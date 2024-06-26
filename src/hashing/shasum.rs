use crate::errors::ShaSumError;
use sha1::Sha1;
use sha2::{Digest, Sha256, Sha512};

pub struct ShaSum<'a> {
    /// Bit length of the checksum
    checksum_type: i32,

    /// Data to process
    data: &'a [u8],
}

impl ShaSum<'_> {
    pub fn new(checksum_type: i32, data: &[u8]) -> Result<ShaSum<'_>, ShaSumError> {
        if ![160, 256, 512].contains(&checksum_type) {
            return Err(ShaSumError::InvalidChecksumType(checksum_type));
        }

        Ok(ShaSum {
            checksum_type,
            data,
        })
    }

    pub fn get_checksum(&self) -> Result<String, ShaSumError> {
        match self.checksum_type {
            512 => { // SHA-512
                let mut hasher = Sha512::new();
                hasher.update(self.data);
                Ok(format!("{:x}", hasher.finalize()))
            }
            256 => { // SHA-256
                let mut hasher = Sha256::new();
                hasher.update(self.data);
                Ok(format!("{:x}", hasher.finalize()))
            }
            160 => { // SHA-1
                let mut hasher = Sha1::new();
                hasher.update(self.data);
                Ok(format!("{:x}", hasher.finalize()))
            }
            _ => Err(ShaSumError::UnexpectedError(
                "Somehow, the options weren't handled correctly.".to_string(),
            )),
        }
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_sha512sum() {
        let data = b"i use arch btw\n";

        let checksummer = ShaSum::new(512, data).unwrap();

        // echo 'i use arch btw' | sha512sum -b
        let expected_checksum = "2ddbe9f9af5a630d3734ce469fac19088e8d0242541768630777de5c56dc4053d346a67527cb95de3ab094d6862f393392ba26bed459d9ad149b423aeae552a2"
            .to_owned();
        let actual_checksum = checksummer.get_checksum().unwrap();
        assert_eq!(actual_checksum, expected_checksum);
    }

    #[test]
    fn test_sha256sum() {
        let data = b"i use arch btw\n";

        let checksummer = ShaSum::new(256, data).unwrap();

        let expected_checksum =
            "80799b90f4c070668b52df31830b60ef767bb039000eec4266f285d498002bb5".to_owned();

        let actual_checksum = checksummer.get_checksum().unwrap();
        assert_eq!(actual_checksum, expected_checksum);
    }

    #[test]
    fn test_sha1sum() {
        let data = b"i use arch btw\n";

        let checksummer = ShaSum::new(160, data).unwrap();

        let expected_checksum = "821609590ef05d00b20c5f4c5a28c56627480eb7".to_owned();

        let actual_checksum = checksummer.get_checksum().unwrap();
        assert_eq!(actual_checksum, expected_checksum);
    }
}
