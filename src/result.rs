//! Megaclite Result
use core::{
    convert::Infallible,
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as DefaultResult,
};
use pairing_ce::GroupDecodingError as GroupDecoding;

/// The custom megaclite error
pub struct Megaclite(String);
impl Display for Megaclite {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&self.0)
    }
}

/// Error generator
macro_rules! error {
    ($($e:ident),*) => {
        /// Sup Error
        #[derive(Debug)]
        #[allow(missing_docs)]
        pub enum Error {
            $($e(String),)+
        }

        impl Display for Error {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
                match self {
                    $(Error::$e(e) => e.fmt(f),)+
                }
            }
        }

        $(
            impl From<$e> for Error {
                fn from(e: $e) -> Error {
                    Error::$e(format!("{}", e))
                }
            }
        )+
    };
}

error! {GroupDecoding, Infallible}

/// Megaclite Result
pub type Result<T> = DefaultResult<T, Error>;
