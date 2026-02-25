use core::fmt::Display;
use thiserror::Error;


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Serde serialization error")]
    SerializationError,

    #[error("Serde deserialization error")]
    DeserializationError,

    #[error("")]
    CapacityError(#[from] heapless::CapacityError),

    #[error("Buffer overflow")]
    BufferOverflow,

    #[cfg(feature = "alloc")]
    #[error("{0}")]
    Serde(alloc::string::String),

    #[error("Unexpected end of input")]
    UnexpectedEof,
    
    #[error("Invalid data type for current state: {0}")]
    DTypeNotSupported(&'static str),

    #[error("Unknown data type: 0x{0:0>2x}")]
    UnknownDType(u8),

    #[error("Serialization not supported for {0} type")]
    SerUnsupported(&'static str),

    #[error("Deserialization not supported for {0} type")]
    DeUnsupported(&'static str),

    #[error("Length required")]
    LengthRequired,
    
    #[error("Missmatch length (expected {expected:?}, found {found:?})")]
    MissmatchLength {
        expected: usize,
        found: usize,
    },

    #[error("Mismatch tuple data type. (expected 0x{expected:0>2x}, found 0x{found:0>2x}). All elements in the tuple must be of the same type.")]
    MissmatchTupleDType {
        expected: u8,
        found: u8,
    },

    #[error("Failed to convert bytes to string")]
    NoUTF8,

    #[error("Expected type: {0}")]
    ExpectedType(&'static str),

    #[error("Root token expected at start")]
    ExpectedRoot,

    #[error("The number of variants in an enum cannot exceed 255")]
    IndexVariantExceeded,

    #[error("Invalid token")]
    InvalidToken,
}

#[cfg(not(feature = "alloc"))]
impl serde::ser::Error for Error {
    fn custom<T: Display>(_msg: T) -> Self {
        Error::SerializationError
    }
}

#[cfg(not(feature = "alloc"))]
impl serde::de::Error for Error {
    fn custom<T: Display>(_msg: T) -> Self {
        Error::DeserializationError
    }
}

#[cfg(feature = "alloc")]
impl serde::ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        use alloc::string::ToString;

        Error::Serde(msg.to_string())
    }
}

#[cfg(feature = "alloc")]
impl serde::de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        use alloc::string::ToString;

        Error::Serde(msg.to_string())
    }
}