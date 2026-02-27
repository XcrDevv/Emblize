pub mod deserializer;
mod de_impl;

use crate::error::Result;
use de_impl::deserialize;


pub fn from_bytes<'de, T>(input: &'de [u8]) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    deserialize::<'de, T>(input)
}