use crate::{core::{token::TokenTag, varint::{varint_usize, varint_usize_max_len}}, error::Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum SerState {
    WriteTypedValue,
    WriteUntypedValue,
    WriteSeqHeader,
    WriteVecHeader,
    WriteUntypedChecked(TokenTag),
}

pub struct Serializer<B: SerializerBuf> {
    pub buf: B,
    pub state: SerState,
}


impl<B: SerializerBuf> Serializer<B> { 
    pub fn new() -> Self{
        Self {
            buf: B::new(),
            state: SerState::WriteTypedValue,
        }
    }

    pub fn write_varint_usize(&mut self, n: usize) -> Result<()> {
        let mut buf = [0u8; varint_usize_max_len()];
        let used = varint_usize(n, &mut buf);
        self.buf.push_bytes(used)
    }
}

pub trait SerializerBuf {
    fn new() -> Self;
    fn push_byte(&mut self, byte: u8) -> Result<()>;
    fn push_bytes(&mut self, data: &[u8]) -> Result<()>;
    fn as_slice(&self) -> &[u8];
}

impl<const N: usize> SerializerBuf for heapless::Vec<u8, N> {
    fn new() -> Self {
        heapless::Vec::new()
    }

    fn push_byte(&mut self, byte: u8) -> Result<()> {
        self.extend_from_slice(&[byte])?;
        Ok(())
    }

    fn push_bytes(&mut self, data: &[u8]) -> Result<()> {
        self.extend_from_slice(data)?;
        Ok(())
    }

    fn as_slice(&self) -> &[u8] {
        self.as_slice()
    }
}

#[cfg(feature = "alloc")]
impl SerializerBuf for alloc::vec::Vec<u8> {
    fn new() -> Self {
        alloc::vec::Vec::new()
    }

    fn push_byte(&mut self, byte: u8) -> Result<()> {
        self.push(byte);
        Ok(())
    }

    fn push_bytes(&mut self, data: &[u8]) -> Result<()> {
        self.extend_from_slice(data);
        Ok(())
    }

    fn as_slice(&self) -> &[u8] {
        self.as_slice()
    }
}