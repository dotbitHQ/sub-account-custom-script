// Import from `core` instead of from `std` since we are in no-std mode
use core::{convert::TryInto, result::Result, slice::from_raw_parts};

// Import heap related library from `alloc`
use alloc::{string::String, vec, vec::Vec};

// Import CKB syscalls and structures
use ckb_std::{ckb_types::prelude::*, cstr_core::CStr, debug};

use crate::{
    error::Error,
    schemas::packed::{AccountCharsReader, SubAccount},
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
                argc >= 5,
                Error::InvalidArgument,
                "The param argc must be greater than or equal to 5."
            );

            let quote = read_u64_param!(args[1]);
            let owner_profit = read_u64_param!(args[2]);
            let das_profit = read_u64_param!(args[3]);

            let mut expected_total_profit = 0;
            // Read the rest of args
            for i in 4..argc {
                let (expiration_years, sub_account_bytes) = read_sub_account_param!(args[i]);

                match SubAccount::from_slice(&sub_account_bytes) {
                    Ok(sub_account) => {
                        let account_len = sub_account.account().len();
                        let account_chars =
                            combine_acocunt_chars(sub_account.account().as_reader());

                        let price_usd = get_price(action, &account_chars, account_len, expiration_years);
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

fn combine_acocunt_chars(account_chars: AccountCharsReader) -> String {
    let mut ret = Vec::new();
    for reader in account_chars.iter() {
        ret.append(&mut reader.bytes().raw_data().to_vec())
    }

    String::from_utf8(ret).unwrap()
}

pub fn usd_to_ckb(usd: u64, quote: u64) -> u64 {
    let total;
    if usd < quote {
        total = usd * 100_000_000 / quote;
    } else {
        total = usd / quote * 100_000_000;
    }

    total
}

fn get_price(_action: &str, _account_chars: &str, account_len: usize, expiration_years: u64) -> u64 {
    let unit_price: u64 = match account_len {
        1 => 16_000_000,
        2 => 8_000_000,
        3 => 4_000_000,
        4 => 2_000_000,
        _ => 1_000_000,
    };

    unit_price * expiration_years
}
