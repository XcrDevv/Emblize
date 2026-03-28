use core::fmt::Display;
use thiserror::Error;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    // --- Serialization ---
    #[error("Serde serialization error")]
    SerializationError,

    #[error("Serialization not supported for {0} type")]
    SerUnsupported(&'static str),

    #[cfg(feature = "alloc")]
    #[error("{0}")]
    Serde(alloc::string::String),

    // --- Deserialization ---
    #[error("Serde deserialization error")]
    DeserializationError,

    #[error("Deserialization not supported for {0} type")]
    DeUnsupported(&'static str),

    // --- Buffer / Capacity ---
    #[error("")]
    CapacityError(#[from] heapless::CapacityError),

    #[error("Buffer overflow")]
    BufferOverflow,

    // --- Parsing / Data format ---
    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Unknown data type: 0x{0:0>2x}")]
    UnknownDType(u8),

    #[error("Varint too large")]
    InvalidVarint,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Root token expected at start")]
    ExpectedRoot,

    #[error("Expected type: {0}")]
    ExpectedType(&'static str),

    // --- Length / Size ---
    #[error("Length required")]
    LengthRequired,

    #[error("Missmatch length (expected {expected:?}, got {got:?})")]
    MissmatchLength {
        expected: usize,
        got: usize,
    },

    // --- Type constraints ---
    #[error("Mismatch tuple data type. (expected 0x{expected:0>2x}, got 0x{got:0>2x}). All elements in the tuple must be of the same type.")]
    HeterogeneousTuple {
        expected: u8,
        got: u8,
    },

    #[error("The number of variants in an enum cannot exceed 255")]
    IndexVariantExceeded,

    // --- Encoding ---
    #[error("Failed to convert bytes to string")]
    NoUTF8,
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