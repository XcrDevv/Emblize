use core::fmt::Debug;
use alloc::boxed::Box;
use alloc::vec;
use alloc::{
    string::String,
    vec::Vec,
    borrow::{ToOwned, Cow}
};

use crate::core::token::{Token, TokenTag};
use crate::de::deserializer::{DeState, Deserializer};
use crate::error::{Error, Result};
use crate::core::{reader::Reader, utils::endian::BytesNum};

impl<'de> Deserializer<'de> {
    pub fn read_any(&mut self) -> Result<Token<'de>> {
        let name = if self.state == DeState::ReadingValue {
            None
        } else {
            Some(Cow::Owned(self.read_string()?))
        };

        let tag = self.input.read_byte()?;

        let token: Token<'de> = match TokenTag::try_from(tag).map_err(|_| Error::UnknownDType(tag))? {
            TokenTag::Bool => Token::Bool(name, self.input.read_byte()? != 0),
            TokenTag::U8 => Token::U8(name, self.input.read_number()?),
            TokenTag::U16 => Token::U16(name, self.input.read_number()?),
            TokenTag::U32 => Token::U32(name, self.input.read_number()?),
            TokenTag::U64 => Token::U64(name, self.input.read_number()?),
            TokenTag::I8 => Token::I8(name, self.input.read_number()?),
            TokenTag::I16 => Token::I16(name, self.input.read_number()?),
            TokenTag::I32 => Token::I32(name, self.input.read_number()?),
            TokenTag::I64 => Token::I64(name, self.input.read_number()?),
            TokenTag::F32 => Token::F32(name, self.input.read_number()?),
            TokenTag::F64 => Token::F64(name, self.input.read_number()?),

            TokenTag::Str => Token::Str(name, Cow::Owned(self.read_string()?)),
            TokenTag::Enum => {
                let is_field = self.state == DeState::ReadingField;
                self.state = DeState::ReadingValue;

                let variant_index = self.input.read_number::<u8>()?;
                let token = if (variant_index & 0x80) != 0 {
                    let token = Box::new(self.read_any().unwrap());
                    Token::Enum(name, variant_index & 0x7F, Some(token))

                } else {
                    Token::Enum(name, variant_index & 0x7F, None)
                };

                if is_field {
                    self.state = DeState::ReadingField
                }

                token
            }

            TokenTag::Struct => {
                self.state = DeState::ReadingField;

                let field_count = self.input.read_byte()? as usize;
                let mut tokens = Vec::with_capacity(field_count);

                for _ in 0..field_count {
                    let token = self.read_any()?;
                    tokens.push(token);
                }
                
                Token::Struct(name, tokens)
            },

            TokenTag::EmptyArr => Token::EmptyArr(name),
            TokenTag::U8Arr => Token::U8Arr(name, self.read_seq()?),
            TokenTag::I32Arr => Token::I32Arr(name, self.read_seq()?),
            TokenTag::I64Arr => Token::I64Arr(name, self.read_seq()?),
            TokenTag::F32Arr => Token::F32Arr(name, self.read_seq()?),
            TokenTag::F64Arr => Token::F64Arr(name, self.read_seq()?),
            TokenTag::StrArr => Token::StrArr(name, self.read_string_seq()?),

            TokenTag::TimestampMillis => Token::TimestampMillis(name, self.input.read_number()?),
            TokenTag::TimestampMicros => Token::TimestampMicros(name, self.input.read_number()?),
            TokenTag::MillisSinceBoot => Token::MillisSinceBoot(name, self.input.read_number()?),
            TokenTag::MicrosSinceBoot => Token::MicrosSinceBoot(name, self.input.read_number()?),
            TokenTag::DurationMillis => Token::DurationMillis(name, self.input.read_number()?),
            TokenTag::DurationMicros => Token::DurationMicros(name, self.input.read_number()?),

            TokenTag::Vec2 => Token::Vec2(name, self.read_fixed_seq()?),
            TokenTag::Vec3 => Token::Vec3(name, self.read_fixed_seq()?),
            TokenTag::Vec4 => Token::Vec4(name, self.read_fixed_seq()?),
            TokenTag::Quat => Token::Quat(name, self.read_fixed_seq()?),
        };

        Ok(token)
    }

    pub fn read_string(&mut self) -> Result<String> {
        let len = self.input.read_number::<u16>()? as usize;
        let string = self.input.read_str_uft8(len)?;
        Ok(string.to_owned())
    }

    fn read_seq<T: BytesNum + Clone>(&mut self) -> Result<Cow<'de, [T]>> {
        let num_size = size_of::<T>();
        let len = self.input.read_number::<u16>()? as usize;

        let mut buf = vec![0; len * num_size];
        self.input.read_exact(&mut buf)?;

        let values: Vec<T> = buf
            .chunks_exact(size_of::<T>())
            .map(|chunk| {
                let bytes = T::Bytes::try_from(chunk).ok().unwrap();
                T::from_be_bytes(bytes)
            })
            .collect();

        Ok(Cow::Owned(values))
    }


    fn read_string_seq(&mut self) -> Result<Cow<'de, [Cow<'de, str>]>> {
        let len = self.input.read_number::<u16>()? as usize;
        let mut values: Vec<Cow<'_, str>> = Vec::with_capacity(len);

        for _ in 0..len {
            let s = self.read_string()?;
            values.push(Cow::Owned(s));
        }

        Ok(Cow::Owned(values))
    }

    fn read_fixed_seq<T: BytesNum + Debug, const N: usize> (
        &mut self,
    ) -> Result<[T; N]> {
        let mut buf = vec![0; N * size_of::<f32>()];
        self.input.read_exact(&mut buf)?;
        let values: Vec<T> = buf
            .chunks_exact(size_of::<T>())
            .map(|chunk| {
                let bytes = T::Bytes::try_from(chunk).ok().unwrap();
                T::from_be_bytes(bytes)
            })
            .collect();

        let arr: [T; N] = values.try_into().unwrap();
        Ok(arr)
    }
}

/// Deserializes a byte slice into an [`OwnedToken`].
///
/// The input must follow the binary format expected by the
/// internal [`Deserializer`]. The function reads from the
/// provided buffer and attempts to reconstruct a dynamic
/// token representation.
///
/// # Errors
///
/// Returns an [`Error`] if:
/// - The input data is malformed or incomplete.
/// - An unexpected token type is encountered.
/// - Any low-level read operation fails.
///
/// # Examples
///
/// ```rust
/// use emblize::dynamic::decode;
///
/// // Example binary payload
/// let bytes: [u8; 33] = [
///     0xA0, 0x02, 0x00, 0x04, 0x66, 0x6C, 0x61, 0x67,
///     0x02, 0x01, 0x00, 0x04, 0x64, 0x61, 0x74, 0x61,
///     0x25, 0x00, 0x02, 0x40, 0x40, 0x00, 0x00, 0x40,
///     0xA0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
///     0x00
/// ];
///
/// let token = decode(&bytes).unwrap();
/// ```
pub fn decode<'a>(bytes: &'a [u8]) -> Result<Token<'a>> {
    let reader = Reader::new(bytes);
    let mut deserializer: Deserializer = Deserializer::new(reader);
    deserializer.read_any()
}