//! Curve Result

use ark_serialize::SerializationError;
use ark_std::{fmt, string::String, vec::Vec};

use parity_scale_codec::{Decode, Encode};

/// Curve Result
pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Curve Error
#[derive(Debug, Encode, Decode)]
pub enum Error {
    InvalidFunctionId,
    SerializeDataFailed,
    ScaleCodecError,
    VerifyParcelFailed,
    Custom(Vec<u8>),
}

impl From<SerializationError> for Error {
    fn from(e: SerializationError) -> Self {
        Error::SerializeDataFailed
    }
}

impl From<parity_scale_codec::Error> for Error {
    fn from(e: parity_scale_codec::Error) -> Self {
        Error::ScaleCodecError
    }
}

impl From<String> for Error {
    fn from(e: String) -> Self {
        Error::Custom(e.as_bytes().to_vec())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidFunctionId => f.write_str("invalid function id"),
            Self::SerializeDataFailed => f.write_str("serialize data failed"),
            Self::ScaleCodecError => f.write_str("scale codec error"),
            Self::VerifyParcelFailed => f.write_str("verify parcel failed"),
            Self::Custom(msg) => write!(f, "{}", String::from_utf8_lossy(msg)),
        }
    }
}

impl Error {
    /// DEBUG string format
    pub fn debug(&self) -> String {
        format!("{:?}", self)
    }
}
