use rustanium_bytes::difficulty_bytes_as_u128;

use crate::hash::BlockHash;

pub struct Miner {}

impl Miner {
    pub fn check_difficulty(hash: &BlockHash, difficulty: u128) -> bool {
        difficulty > difficulty_bytes_as_u128(hash.as_ref())
    }
}
