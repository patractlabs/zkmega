//! Matter Lab Result
use bellman_ce::{
    pairing::{ff::PrimeFieldDecodingError, GroupDecodingError},
    SynthesisError,
};
use core::fmt::{Display, Formatter, Result as FmtResult};

/// Matter Labs Result
pub type Result<T> = core::result::Result<T, Error>;

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

error! {PrimeFieldDecodingError, GroupDecodingError, SynthesisError}
