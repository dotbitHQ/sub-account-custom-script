// script-001
pub const WITNESS_HEADER: [u8; 10] = [115, 99, 114, 105, 112, 116, 45, 48, 48, 49];
pub const WITNESS_HEADER_BYTES: usize = WITNESS_HEADER.len();
pub const WITNESS_VERSION_BYTES: usize = 4;

pub const CKB_HASH_DIGEST: usize = 32;
pub const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";
