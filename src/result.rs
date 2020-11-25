//! Megaclite Result
use bellman_ce::{
    pairing::{
        ff::PrimeFieldDecodingError as PrimeFieldDecoding, GroupDecodingError as GroupDecoding,
    },
    SynthesisError as Synthesis,
};
use core::array::TryFromSliceError;
use core::{
    fmt::{Display, Formatter, Result as FmtResult},
    result::Result as DefaultResult,
};
use num_bigint::ParseBigIntError as ParseBigInt;

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

error! {PrimeFieldDecoding, GroupDecoding, Synthesis,ParseBigInt,Megaclite,TryFromSliceError}

/// Megaclite Result
pub type Result<T> = DefaultResult<T, Error>;
