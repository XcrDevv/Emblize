pub mod serializer;
mod ser_impl;

use serde::Serialize;
use crate::error::Result;
use ser_impl::serialize;

pub fn to_heaplessvec<T, const N: usize>(value: &T) -> Result<heapless::Vec<u8, N>>
where
    T: Serialize,
{
    serialize(value)
}

#[cfg(feature = "alloc")]
pub fn to_allocvec<T>(value: &T) -> Result<alloc::vec::Vec<u8>>
where
    T: Serialize,
{
    serialize(value)
}