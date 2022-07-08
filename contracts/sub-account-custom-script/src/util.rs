use crate::{
    constants::*,
    schemas::packed::*
};
use alloc::string::String;
use blake2b_ref::Blake2bBuilder;
use molecule::{bytes, prelude::*};

macro_rules! impl_uint_convert {
    ($uint_type:ty, $mol_type:ty, $mol_reader_typer:ident, $length: expr) => {
        impl From<$uint_type> for $mol_type {
            fn from(v: $uint_type) -> Self {
                Self::new_unchecked(bytes::Bytes::from(v.to_le_bytes().to_vec()))
            }
        }

        impl From<$mol_type> for $uint_type {
            fn from(v: $mol_type) -> Self {
                let mut buf = [0u8; $length];
                buf.copy_from_slice(v.raw_data().as_ref());
                <$uint_type>::from_le_bytes(buf)
            }
        }

        impl From<$mol_reader_typer<'_>> for $uint_type {
            fn from(v: $mol_reader_typer<'_>) -> Self {
                let mut buf = [0u8; $length];
                buf.copy_from_slice(v.raw_data());
                <$uint_type>::from_le_bytes(buf)
            }
        }
    };
}

impl_uint_convert!(u8, Uint8, Uint8Reader, 1);
impl_uint_convert!(u32, Uint32, Uint32Reader, 4);
impl_uint_convert!(u64, Uint64, Uint64Reader, 8);

pub fn blake2b_256<T: AsRef<[u8]>>(s: T) -> [u8; 32] {
    let mut result = [0u8; CKB_HASH_DIGEST];
    let mut blake2b = Blake2bBuilder::new(CKB_HASH_DIGEST)
        .personal(CKB_HASH_PERSONALIZATION)
        .build();
    blake2b.update(s.as_ref());
    blake2b.finalize(&mut result);
    result
}

pub fn combine_acocunt_chars(account_chars: AccountCharsReader) -> String {
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
