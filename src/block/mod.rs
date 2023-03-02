use crate::{
    hash::{format_block_hash, BlockHash},
    miner,
    traits::hashable::Hashable,
};
use rustanium_bytes::{u128_bytes, u32_bytes, u64_bytes};
use rustanium_time::now;
use std::fmt::{Debug, Formatter, Result as FormatResult};

pub type BlockIndex = u32;
pub type BlockNonce = u64;
pub type BlockTimestamp = u128;
pub type BlockPayload = String;
pub type BlockDifficulty = u128;

pub struct Block {
    pub index: BlockIndex,
    pub hash: BlockHash,
    pub prev_block_hash: BlockHash,
    pub nonce: BlockNonce,
    pub timestamp: BlockTimestamp,
    pub payload: BlockPayload,
    pub difficulty: BlockDifficulty,
}

impl Block {
    pub fn new_genesis() -> Self {
        Block {
            index: 0,
            hash: vec![0; 32],
            prev_block_hash: vec![0; 32],
            nonce: 0,
            timestamp: now(),
            payload: String::from("Genesis block"),
            difficulty: 0x00ffffffffffffffffffffffffffffff,
        }
    }

    pub fn new(
        index: BlockIndex,
        prev_block_hash: BlockHash,
        nonce: BlockNonce,
        difficulty: BlockDifficulty,
        payload: BlockPayload,
    ) -> Self {
        Block {
            index,
            hash: vec![0; 32],
            prev_block_hash,
            nonce,
            timestamp: now(),
            difficulty,
            payload,
        }
    }

    fn set_nonce(&mut self, nonce: BlockNonce) -> &BlockNonce {
        self.nonce = nonce;
        &self.nonce
    }

    pub fn mine(&mut self) {
        println!("");
        for current_nonce in 0..(u64::MAX) {
            self.set_nonce(current_nonce);
            let hash = self.hash();
            println!("Trying... {}", &format_block_hash(&hash));
            if miner::Miner::check_difficulty(&hash, self.difficulty) {
                println!("");
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_bytes(&self.index));
        bytes.extend(&u64_bytes(&self.nonce));
        bytes.extend(&u128_bytes(&self.timestamp));
        bytes.extend(&self.prev_block_hash);
        bytes.extend(self.payload.as_bytes());
        bytes
    }
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(
            f,
            "=========== Block #{} ===========\nHash: {}\nPrevious block hash: {}\nNonce: {}\nTimestamp: {}\nDifficulty: {}\nPayload: {}\n",
            &self.index, format_block_hash(&self.hash), format_block_hash(&self.prev_block_hash), &self.nonce, &self.timestamp, &self.difficulty, &self.payload
        )
    }
}
