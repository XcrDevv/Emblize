pub mod serializer;
mod ser_impl;

use serde::Serialize;
use crate::error::Result;
use ser_impl::serialize;

/// Serializes a value into a [`heapless::Vec`] using this crate’s
/// `serde` serializer.
///
/// Unlike [`encode`], which operates on dynamic [`Token`] values,
/// this function works with any type implementing [`serde::Serialize`],
/// allowing usage with `#[derive(Serialize)]`.
///
/// The output buffer is backed by a fixed-capacity [`heapless::Vec`],
/// making this function suitable for `no_std` environments without
/// a global allocator.
///
/// # Type Parameters
///
/// - `T`: The type to serialize.
/// - `N`: The capacity (in bytes) of the resulting buffer.
///
/// # Errors
///
/// Returns an error if:
/// - Serialization fails.
/// - The serialized representation exceeds the capacity `N`.
///
/// # Examples
///
/// ```rust
/// use serde::Serialize;
/// use emblize::to_heaplessvec;
///
/// #[derive(Serialize)]
/// struct Data {
///     flag: u8,
/// }
///
/// let value = Data { flag: 1 };
/// let bytes = to_heaplessvec::<_, 32>(&value).unwrap();
/// ```
pub fn to_heaplessvec<T, const N: usize>(value: &T) -> Result<heapless::Vec<u8, N>>
where
    T: Serialize,
{
    serialize(value)
}

/// Serializes a value into an [`alloc::vec::Vec`] using this crate’s
/// `serde` serializer.
///
/// Unlike [`encode`], which serializes dynamic [`Token`] values,
/// this function works with any type implementing [`serde::Serialize`],
/// enabling seamless integration with `#[derive(Serialize)]`.
///
/// This variant requires the `alloc` feature and returns a
/// dynamically-sized buffer.
///
/// # Errors
///
/// Returns an error if serialization fails.
///
/// # Examples
///
/// ```rust
/// use serde::Serialize;
/// use emblize::to_allocvec;
///
/// #[derive(Serialize)]
/// struct Data {
///     flag: u8,
/// }
///
/// let value = Data { flag: 1 };
/// let bytes = to_allocvec(&value).unwrap();
/// ```
#[cfg(feature = "alloc")]
pub fn to_allocvec<T>(value: &T) -> Result<alloc::vec::Vec<u8>>
where
    T: Serialize,
{
    serialize(value)
}