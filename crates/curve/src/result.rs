//! Curve Result
use ark_serialize::SerializationError;
use ark_std::string::String;
use core::result::Result as BasicResult;
use parity_scale_codec::{Decode, Encode, Error as CodecError};

/// Curve Result
pub type Result<T> = BasicResult<T, Error>;

/// Curve Error
#[derive(Debug, Encode, Decode)]
pub enum Error {
    InvalidFunctionId,
    SerializeDataFailed,
    ScaleCodecError,
    VerifyParcelFailed,
    Custom(String),
}

impl From<SerializationError> for Error {
    fn from(e: SerializationError) -> Self {
        Error::SerializeDataFailed
    }
}

impl From<CodecError> for Error {
    fn from(e: CodecError) -> Self {
        Error::ScaleCodecError
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Custom(e)
    }
}

impl Error {
    /// DEBUG string format
    pub fn debug(&self) -> String {
        format!("{:?}", self)
    }
}
