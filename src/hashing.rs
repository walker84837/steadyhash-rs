use openssl::hash::{Hasher, MessageDigest};
use std::fmt::Write;

pub struct Sha256Sum;

impl Sha256Sum {
    pub fn get_checksum(data: &[u8]) -> String {
        let mut hasher = Hasher::new(MessageDigest::sha256())
            .expect("Failed to create a digest");

        hasher.update(data).expect("Failed to update digest with data");
        let result = hasher.finish().expect("Failed to generate checksum");

        let mut checksum = String::with_capacity(result.len() * 2);
        result.iter().for_each(|byte| {
            write!(checksum, "{:02x}", byte).expect("Failed to write to string");
        });
        checksum
    }
}