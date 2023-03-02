pub type BlockHash = Vec<u8>;

pub fn format_block_hash(hash: &BlockHash) -> String {
    hex::encode(hash)
}
