// Import from `core` instead of from `std` since we are in no-std mode
use core::result::Result;

// Import heap related library from `alloc`
// https://doc.rust-lang.org/alloc/index.html
use alloc::{vec, vec::Vec};
use core::convert::TryInto;

// Import CKB syscalls and structures
// https://nervosnetwork.github.io/ckb-std/riscv64imac-unknown-none-elf/doc/ckb_std/index.html
use ckb_std::{
    debug,
    high_level::{load_script, load_tx_hash},
    ckb_types::{bytes::Bytes, prelude::*},
};
use ckb_std::cstr_core::CStr;

use crate::error::Error;

/// This is the entry of the contract.
///
/// - argc means the count of params passed to the contract.
/// - argv is a point of point, which point to an array of points which point to params.
pub fn main(argc: usize, argv: *const *const u8) -> Result<(), Error> {

    Ok(())
}

