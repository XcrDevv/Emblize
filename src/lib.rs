#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod platform;

pub mod core;
pub mod de;
pub mod error;
pub mod macros;
pub mod ser;

#[cfg(feature = "alloc")]
pub use platform::{
    builder::{StructBuilder},
    serializer::as_bytes,
    deserializer::from_bytes,
};

pub use ser::ser_impl::serialize;
pub use de::de_impl::deserialize;