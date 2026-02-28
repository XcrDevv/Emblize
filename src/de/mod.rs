pub mod deserializer;
mod de_impl;

use crate::error::Result;
use de_impl::deserialize;

/// Deserializes a byte slice into a strongly-typed value using this
/// crate’s `serde` deserializer.
///
/// Unlike [`decode`], which produces a dynamic [`Token`] representation,
/// this function reconstructs any type implementing
/// [`serde::Deserialize`], making it suitable for use with
/// `#[derive(Deserialize)]`.
///
/// This is the typed counterpart to [`to_heaplessvec`] and
/// [`to_allocvec`].
///
/// # Type Parameters
///
/// - `T`: The target type to deserialize.
///
/// # Errors
///
/// Returns an error if:
/// - The input does not follow the expected binary format.
/// - Deserialization into `T` fails.
/// - The input data is incomplete or malformed.
///
/// # Examples
///
/// ```rust
/// use serde::{Serialize, Deserialize};
/// use emblize::{to_allocvec, from_bytes};
///
/// #[derive(Serialize, Deserialize, PartialEq, Debug)]
/// struct Data {
///     flag: u8,
/// }
///
/// let original = Data { flag: 1 };
/// let bytes = to_allocvec(&original).unwrap();
/// let decoded: Data = from_bytes(&bytes).unwrap();
///
/// assert_eq!(original, decoded);
/// ```
pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    deserialize::<'de, T>(input)
}