//! Generated by capsule
//!
//! `main.rs` is used to define rust lang items and modules.
//! See `entry.rs` for the `main` function.
//! See `error.rs` for the `Error` type.

#![no_std]
#![no_main]
#![feature(asm_sym)]
#![feature(lang_items)]
#![feature(alloc_error_handler)]
#![feature(panic_info_message)]

// define modules
#[macro_use]
mod macros;
mod entry;
mod error;
mod schemas;

use ckb_std::default_alloc;
use core::arch::asm;

ckb_std::entry!(program_entry);
default_alloc!();

/// program entry
///
///  Both `argc` and `argv` can be omitted.
fn program_entry(_argc: usize, _argv: *const *const u8) -> i8 {
    // Call main function and return error code
    match entry::main(_argc, _argv) {
        Ok(_) => 0,
        Err(err) => err as i8,
    }
}
