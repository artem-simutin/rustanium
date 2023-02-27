use std::fmt::{Display, Formatter, Result as FormatResult};

pub struct BlockHash(pub Vec<u8>);

impl BlockHash {
    pub fn get(&self) -> &Vec<u8> {
        &self.0
    }
}

impl AsRef<[u8]> for BlockHash {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for BlockHash {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        write!(f, "{}", &hex::encode(&self))
    }
}
