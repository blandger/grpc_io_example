use std::{error, fmt, result};
use std::fmt::Display;
use grpcio::RpcStatus;
use protobuf::ProtobufError;
// use grpcio::Error as grpcError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    /// Codec error.
    Codec(Box<dyn error::Error + Send + Sync>),
    /// Rpc request fail.
    RpcFailure(RpcStatus),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RpcFailure(s) => {
                if s.message().is_empty() {
                    write!(fmt, "RpcFailure: {}", s.code())
                } else {
                    write!(fmt, "RpcFailure: {} {}", s.code(), s.message())
                }
            }
            other_error => write!(fmt, "{other_error:?}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            Error::Codec(ref e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

#[cfg(any(feature = "protobuf-codec", feature = "protobufv3-codec"))]
impl From<ProtobufError> for Error {
    fn from(e: ProtobufError) -> Error {
        Error::Codec(Box::new(e))
    }
}

impl From<grpcio::Error> for Error {
    fn from(e: grpcio::Error) -> Error {
        Error::Codec(Box::new(e))
    }
}