use crate::errors::B2SumError;
use blake2::{Blake2b512, Blake2s256, Digest};

pub struct Blake2b<'a> {
    /// Bit length of the checksum
    checksum_type: i32,

    /// Data to process
    data: &'a [u8],
}

impl Blake2b<'_> {
    pub fn new(checksum_type: i32, data: &[u8]) -> Result<Blake2b<'_>, B2SumError> {
        if ![256, 512].contains(&checksum_type) {
            return Err(B2SumError::InvalidChecksumType(checksum_type));
        }

        Ok(Blake2b {
            checksum_type,
            data,
        })
    }

    pub fn get_checksum(&self) -> Result<String, B2SumError> {
        match self.checksum_type {
            512 => {
                let mut hasher = Blake2b512::new();
                hasher.update(self.data);
                Ok(format!("{:x}", hasher.finalize()))
            }
            256 => {
                let mut hasher = Blake2s256::new();
                hasher.update(self.data);
                Ok(format!("{:x}", hasher.finalize()))
            }
            _ => Err(B2SumError::UnexpectedError(
                "invalid b2sum type".to_string(),
            )),
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;
}
