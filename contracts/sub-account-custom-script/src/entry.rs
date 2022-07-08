// Import from `core` instead of from `std` since we are in no-std mode
use core::{convert::TryInto, result::Result, slice::from_raw_parts};

// Import heap related library from `alloc`
use alloc::vec;
use alloc::vec::Vec;

// Import CKB syscalls and structures
use ckb_std::ckb_constants::Source;
use ckb_std::error::SysError;
use ckb_std::{ckb_types::prelude::*, cstr_core::CStr, debug, syscalls};

use molecule::hex_string;

use crate::{
    constants::*,
    error::Error,
    schemas::packed::{PriceConfigList, SubAccount},
    util::*,
};

/// This is the entry of the contract.
///
/// - argc means the count of params passed to the contract.
/// - argv is a point of point, which point to an array of points which point to params.
pub fn main(argc: usize, argv: *const *const u8) -> Result<(), Error> {
    debug!("====== Running sub-account-custom-script ======");

    // The argc will have at least 4 params.
    das_assert!(
        argc >= 1,
        Error::InvalidArgument,
        "The param argc must be greater than or equal to 1."
    );

    // Retrieve params from argv, your may treat the args as the params array which looks like:
    // [
    //     [action: str],
    //     [quote: uint64],
    //     [owner_profit: uint64],
    //     [das_profit: uint64],
    //     [expiration_years: uint32][sub_account: SubAccount],
    //     ...
    // ]
    let args = unsafe { from_raw_parts(argv, argc as usize) };
    // Read the special leading args one by one
    let action = unsafe { CStr::from_ptr(args[0]).to_str().unwrap() };

    match action {
        "create_sub_account" | "renew_sub_account" => {
            // The argc will have at least 4 params.
            das_assert!(
                argc >= 6,
                Error::InvalidArgument,
                "The param argc must be greater than or equal to 5."
            );

            debug!("Parsing arguments from args ...");

            let quote = read_u64_param!(args[1]);
            let owner_profit = read_u64_param!(args[2]);
            let das_profit = read_u64_param!(args[3]);
            let witness_hash = read_bytes_param!(args[4]);

            debug!("Finding witness of configuration ...");

            let mut i = 0;
            let mut config_opt = None;
            loop {
                let mut buf = [0u8; WITNESS_HEADER_BYTES];
                let ret = syscalls::load_witness(&mut buf, 0, i, Source::Input);
                match ret {
                    Ok(_) => i += 1,
                    Err(SysError::LengthNotEnough(size)) => {
                        if &buf != &WITNESS_HEADER {
                            i += 1;
                            continue;
                        }

                        let mut buf = vec![0u8; size];
                        syscalls::load_witness(&mut buf, 0, i, Source::Input)?;
                        let body = &buf[(WITNESS_HEADER_BYTES + WITNESS_VERSION_BYTES)..];
                        let hash = blake2b_256(&body);

                        debug!("witness.body: 0x{}", hex_string(&body));
                        debug!("hash of witness.hash: 0x{}", hex_string(&hash));

                        das_assert!(
                            &witness_hash == &hash[0..10],
                            Error::WitnessHashMismatch,
                            "The hash of witness body is not matched.(hash_in_args: 0x{}, hash_calculated: 0x{})",
                            hex_string(&witness_hash),
                            hex_string(&hash[0..10])
                        );

                        match PriceConfigList::from_slice(body) {
                            Ok(val) => {
                                config_opt = Some(val);
                            }
                            Err(_) => {
                                das_assert!(
                                    false,
                                    Error::WitnessDecodingError,
                                    "Can not decoding witnesses[{}] as table PriceConfigList.",
                                    i
                                );
                            }
                        }

                        break;
                    }
                    Err(SysError::IndexOutOfBound) => {
                        das_assert!(
                            false,
                            Error::CanNotFindWitness,
                            "Can not find the witness of configuration."
                        );
                    }
                    Err(e) => return Err(Error::from(e)),
                }
            }

            let configs = match config_opt {
                Some(configs) => {
                    let mut ret = Vec::new();
                    let mut prev_len = 0;
                    for config in configs.into_iter() {
                        let len = u8::from(config.length());

                        das_assert!(
                            len > prev_len,
                            Error::ConfigValueError,
                            "The length in the configuration should be incremental."
                        );

                        let new_price = u64::from(config.new());
                        let renew_price = u64::from(config.renew());

                        prev_len = len;
                        ret.push((len, new_price, renew_price));
                    }

                    ret
                }
                None => {
                    debug!("Can not find the witness of configuration.");
                    return Err(Error::CanNotFindWitness);
                }
            };

            let mut expected_total_profit = 0;
            // Read the rest of args
            for i in 5..argc {
                let (expiration_years, sub_account_bytes) = read_sub_account_param!(args[i]);

                match SubAccount::from_slice(&sub_account_bytes) {
                    Ok(sub_account) => {
                        let account_len = sub_account.account().len();
                        let account_chars =
                            combine_acocunt_chars(sub_account.account().as_reader());

                        let price_usd = get_price(
                            action,
                            &configs,
                            &account_chars,
                            account_len,
                            expiration_years,
                        );
                        let price_ckb = usd_to_ckb(price_usd, quote);

                        expected_total_profit += price_ckb;
                    }
                    Err(_err) => {
                        debug!("Decoding SubAccount from slice failed: {}", _err);
                        return Err(Error::InvalidSubAccountData);
                    }
                }
            }

            das_assert!(
                owner_profit + das_profit >= expected_total_profit,
                Error::InvalidProfit,
                "The total profit should be {} shannon, but only {} found.",
                expected_total_profit,
                owner_profit + das_profit
            );
        }
        "get_price" => todo!(),
        _ => return Err(Error::ActionNotSupported),
    }

    Ok(())
}

fn get_price(
    action: &str,
    configs: &[(u8, u64, u64)],
    _account_chars: &str,
    account_len: usize,
    expiration_years: u64,
) -> u64 {
    let mut unit_price = 0;
    let mut max_length = 0;
    for config in configs {
        max_length = config.0;
        if account_len == config.0 as usize {
            if action == "create_sub_account" {
                unit_price = config.1;
            } else {
                unit_price = config.2;
            }
        }
    }

    if account_len >= max_length as usize {
        unit_price = if action == "create_sub_account" {
            configs[configs.len() - 1].1
        } else {
            configs[configs.len() - 1].2
        };
    }

    unit_price * expiration_years
}
