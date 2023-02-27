use crate::hash::BlockHash;
use crypto_hash::{digest, Algorithm::SHA256};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;

    fn hash(&self) -> BlockHash {
        BlockHash(digest(SHA256, &self.bytes()))
    }
}
