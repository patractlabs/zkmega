//! Curve Result
use ark_serialize::SerializationError;
use ark_std::string::String;
use core::result::Result as BasicResult;

#[cfg(feature = "ink")]
use ink_env::Error as InkError;

/// Curve Result
pub type Result<T> = BasicResult<T, Error>;

/// Curve Error
#[derive(Debug)]
pub enum Error {
    Serialize(SerializationError),
    Custom(String),
    #[cfg(feature = "ink")]
    InkError(InkError),
}

impl From<SerializationError> for Error {
    fn from(e: SerializationError) -> Self {
        Error::Serialize(e)
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

#[cfg(feature = "ink")]
impl From<InkError> for Error {
    fn from(e: InkError) -> Self {
        Error::InkError(e)
    }
}
