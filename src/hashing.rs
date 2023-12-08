use sha2::{Digest, Sha256};

pub struct Sha256Sum;

impl Sha256Sum {
    pub fn get_checksum(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }
}

mod tests {
    // use super::*;

    #[test]
    fn test_checksum() {
        use crate::hashing::Sha256Sum;

        let data = b"i use arch btw\n";

        let expected_checksum =
            "80799b90f4c070668b52df31830b60ef767bb039000eec4266f285d498002bb5".to_owned();

        let actual_checksum = Sha256Sum::get_checksum(data);
        assert_eq!(actual_checksum, expected_checksum);
    }
}
