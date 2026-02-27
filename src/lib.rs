#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod dynamic;

pub mod core;
pub mod de;
pub mod error;
pub mod macros;
pub mod ser;

// #[cfg(feature = "alloc")]
// pub use dynamic::{
//     builder::{StructBuilder},
//     encoder::encode,
//     decoder::decode,
// };

pub use ser::{to_allocvec, to_heaplessvec};
pub use de::{from_bytes};
pub use core::types;