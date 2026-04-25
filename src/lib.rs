//! # Emblize binary format.
//!
//! This module provides strongly-typed serialization and deserialization
//! support for the **emblize** binary format using [`serde`].
//!
//! Unlike the dynamic API based on [`Token`] and builders, this interface
//! operates on statically-defined Rust types and is intended to be used
//! with `#[derive(Serialize, Deserialize)]`.
//!
//! ## Embedded-focused design
//!
//! **emblize** is a compact binary format designed primarily for
//! embedded systems, with `no_std` support by default.
//!
//! - `heapless` support enables fixed-capacity buffers without requiring
//!   a global allocator.
//! - The optional `alloc` feature enables dynamic containers such as
//!   `Vec` and `String`.
//! - The `std` feature automatically enables `alloc` and provides full
//!   standard library integration.
//!
//! Although optimized for embedded environments, the format can also
//! be used on desktop and server platforms when `alloc` or `std`
//! features are enabled.
//!
//! ## Built-in types
//!
//! The crate includes several embedded-friendly newtypes, such as:
//!
//! - `Vec2`, `Vec3`, `Vec4`
//! - Time-related types like `TimestampMillis`, `MillisSinceBoot`,
//!   `DurationMillis`, and their microsecond variants.
//!
//! These types are lightweight wrappers (newtype structs) without
//! additional runtime behavior, designed for consistent and
//! predictable binary encoding within the emblize format.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(feature = "alloc")]
pub mod dynamic;

#[cfg(feature = "frame")]
pub mod frame;

pub mod core;
pub mod de;
pub mod error;
pub mod macros;
pub mod ser;

#[cfg(feature = "alloc")]
pub use ser::to_allocvec;

pub use ser::to_heaplessvec;
pub use de::{from_bytes};
pub use core::types;

#[cfg(feature = "frame")]
pub use frame::frame_parser::FrameParser;