use crate::{core::reader::Reader, error::{Error, Result}};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DeState {
    ReadUntypedValue,
    ReadTypedValue,
    ReadSeq(Option<u8>),
    ReadVec,
}

pub struct Deserializer<'de> {
    pub input: Reader<'de>,
    pub state: DeState,
}

impl<'de> Deserializer<'de> {
    pub fn new(input: Reader<'de>) -> Self {
        Self {
            input,

            state: DeState::ReadUntypedValue,
        }
    }

    #[cfg(target_pointer_width = "16")]
    #[inline(always)]
    pub fn read_variant_usize(&mut self) -> Result<usize> {
        self.read_variant_n::<16>()
    }

    #[cfg(target_pointer_width = "32")]
    #[inline(always)]
    pub fn read_variant_usize(&mut self) -> Result<usize> {
        self.read_variant_n::<32>()
    }

    #[cfg(target_pointer_width = "64")]
    #[inline(always)]
    pub fn read_variant_usize(&mut self) -> Result<usize> {
        self.read_variant_n::<64>()
    }

    #[inline]
    fn read_variant_n<const N: usize>(&mut self) -> Result<usize> {
        let mut result: usize = 0;
        let mut shift: usize = 0;

        let mut buf = [0u8; 1];

        loop {
            self.input.read_exact(&mut buf)?;
            let byte = buf[0];

            result |= ((byte & 0x7F) as usize) << shift;

            if byte < 0x80 {
                return Ok(result);
            }

            shift += 7;

            if shift >= N {
                return Err(Error::InvalidVarint);
            }
        }
    }
}