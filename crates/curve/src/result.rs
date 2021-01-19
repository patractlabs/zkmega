//! Curve Result
use ark_serialize::SerializationError;
use ark_std::string::String;
use core::result::Result as BasicResult;
use parity_scale_codec::{Decode, Encode, Error as CodecError};

// #[cfg(feature = "ink")]
// use ink_env::Error as InkError;

/// Curve Result
pub type Result<T> = BasicResult<T, Error>;

/// Curve Error
#[derive(Debug, Encode, Decode)]
pub enum Error {
    Custom(String),
}

impl From<SerializationError> for Error {
    fn from(e: SerializationError) -> Self {
        Error::Custom("Serialize data failed".into())
    }
}

impl From<&'static str> for Error {
    fn from(e: &'static str) -> Self {
        Error::Custom(String::from(e))
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Custom(e)
    }
}

impl From<CodecError> for Error {
    fn from(e: CodecError) -> Self {
        Error::Custom("Scale codec error".into())
    }
}

// #[cfg(feature = "ink")]
// impl From<InkError> for Error {
//     fn from(e: InkError) -> Self {
//         Error::InkError(e)
//     }
// }

impl Error {
    /// DEBUG string format
    pub fn debug(&self) -> String {
        format!("{:?}", self)
    }
}
