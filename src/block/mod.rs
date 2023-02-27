use crate::{hash::BlockHash, traits::hashable::Hashable};
use rustanium_bytes::{u128_bytes, u32_bytes, u64_bytes};
use rustanium_time::now;
use std::fmt::{Debug, Formatter, Result as FormatResult};

pub type BlockIndex = u32;
pub type BlockNonce = u64;
pub type BlockTimestamp = u128;
pub type BlockPayload = String;

pub struct Block {
    pub index: BlockIndex,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: BlockNonce,
    pub timestamp: BlockTimestamp,
    pub payload: BlockPayload,
}

impl Block {
    pub fn new_genesis() -> Self {
        Block {
            index: 0,
            hash: BlockHash(vec![0; 32]),
            prev_block_hash: BlockHash(vec![0; 32]),
            nonce: 0,
            timestamp: now(),
            payload: String::from("Genesis block"),
        }
    }

    pub fn new(
        index: BlockIndex,
        prev_block_hash: BlockHash,
        nonce: BlockNonce,
        payload: BlockPayload,
    ) -> Self {
        Block {
            index,
            hash: BlockHash(vec![0; 32]),
            prev_block_hash,
            nonce,
            timestamp: now(),
            payload,
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(self.prev_block_hash.get());
        bytes.extend(self.payload.as_bytes());
        bytes
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(
            f,
            "=========== Block #{} ===========\nHash: {}\nPrevious block hash: {}\nNonce: {}\nTimestamp: {}\nPayload: {}",
            &self.index, &self.hash, &self.prev_block_hash, &self.nonce, &self.timestamp, &self.payload
        )
    }
}
