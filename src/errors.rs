use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShaSumError {
    /// Error indicating that an invalid SHA checksum type has been provided.
    #[error("Invalid checksum type 'SHA-{0}'. The only supported types are SHA1, SHA256 and SHA512")]
    InvalidChecksumType(i32),

    /// Error indicating that something went wrong when it shouldn't. Alternative to panic!()
    #[error("Something went unexpectedly wrong: {0}.")]
    UnexpectedError(String),
}

#[derive(Error, Debug)]
pub enum B2SumError {
    /// Error indicating that an invalid Blake checksum type has been provided. 
    #[error("Invalid checksum type 'Blake2b-{0}'. The only supported types are Blake2b-256 and Blake2b-512.")]
    InvalidChecksumType(i32),

    /// Error indicating that something went wrong when it shouldn't. Alternative to panic!()
    #[error("Something went unexpectedly wrong: {0}.")]
    UnexpectedError(String),
}
