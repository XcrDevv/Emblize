use serde::Serialize;

use crate::{error::Result, serialize};

pub mod serializer;
pub mod ser_impl;

pub fn serialize_to_vec<T, const N: usize>(value: &T) -> Result<heapless::Vec<u8, N>>
where
    T: Serialize,
{
    serialize(value)
}

#[cfg(feature = "alloc")]
pub fn serialize_to_alloc_vec<T>(value: &T) -> Result<alloc::vec::Vec<u8>>
where
    T: Serialize,
{
    serialize(value)
}