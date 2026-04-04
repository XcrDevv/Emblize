use alloc::boxed::Box;
use alloc::vec;
use alloc::{
    string::String,
    vec::Vec,
    borrow::{ToOwned, Cow}
};
use num_enum::TryFromPrimitive;

use crate::core::token::{Token, TokenTag};
use crate::de::deserializer::{DeState, Deserializer};
use crate::error::{Error, Result};
use crate::core::{reader::Reader, utils::endian::BytesNum};

impl<'de> Deserializer<'de> {
    pub fn read_any(&mut self) -> Result<Token<'de>> {
        let name = match self.state {
            DeState::ReadTypedValue => Some(Cow::Owned(self.read_string()?)),
            _ => None,
        };

        let tag = match self.state {
            DeState::ReadSeq(t)  => t.unwrap(),
            _ => self.input.read_byte()?
        };

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
                let is_field = self.state == DeState::ReadTypedValue;
                self.state = DeState::ReadUntypedValue;

                let variant_index = self.input.read_number::<u8>()?;
                let token = if (variant_index & 0x80) != 0 {
                    let token = Box::new(self.read_any().unwrap());
                    Token::Enum(name, variant_index & 0x7F, Some(token))

                } else {
                    Token::Enum(name, variant_index & 0x7F, None)
                };

                if is_field {
                    self.state = DeState::ReadTypedValue
                }

                token
            }
            TokenTag::Some => {
                let is_field = self.state == DeState::ReadTypedValue;
                self.state = DeState::ReadUntypedValue;

                let token = Box::new(self.read_any().unwrap());
                
                if is_field {
                    self.state = DeState::ReadTypedValue;
                }

                Token::Some(name, token)
            }
            TokenTag::None => {
                let is_field = self.state == DeState::ReadTypedValue;
                self.state = DeState::ReadUntypedValue;

                if is_field {
                    self.state = DeState::ReadTypedValue;
                }
                
                Token::None(name)
            }

            TokenTag::Struct => {
                self.state = DeState::ReadTypedValue;

                let field_count = self.input.read_byte()? as usize;
                let mut tokens = Vec::with_capacity(field_count);

                for _ in 0..field_count {
                    let token = self.read_any()?;
                    tokens.push(token);
                }
                
                Token::Struct(name, tokens)
            },

            TokenTag::EmptyArr => Token::EmptyArr(name),
            TokenTag::Array => {
                let size = self.read_variant_usize()?;
                let mut tokens = Vec::with_capacity(size);
                let arr_type = TokenTag::try_from_primitive(self.input.read_byte()?)
                .map_err(|_| Error::InvalidToken)?;
            
                let prev = self.state;
                self.state = DeState::ReadSeq(Some(arr_type as u8));
                for _ in 0..size {
                    let token = self.read_any()?;
                    tokens.push(token);
                }
                self.state = prev;
                
                Token::Array(name, arr_type, tokens)
            }
            TokenTag::Bytes => Token::Bytes(name, self.read_seq()?),

            TokenTag::TimestampMillis => Token::TimestampMillis(name, self.input.read_number()?),
            TokenTag::TimestampMicros => Token::TimestampMicros(name, self.input.read_number()?),
            TokenTag::MillisSinceBoot => Token::MillisSinceBoot(name, self.input.read_number()?),
            TokenTag::MicrosSinceBoot => Token::MicrosSinceBoot(name, self.input.read_number()?),
            TokenTag::DurationMillis => Token::DurationMillis(name, self.input.read_number()?),
            TokenTag::DurationMicros => Token::DurationMicros(name, self.input.read_number()?),

            TokenTag::Vec2 => Token::Vec2(name, Box::new(self.read_fixed_seq()?)),
            TokenTag::Vec3 => Token::Vec3(name, Box::new(self.read_fixed_seq()?)),
            TokenTag::Vec4 => Token::Vec4(name, Box::new(self.read_fixed_seq()?)),
            TokenTag::Quat => Token::Quat(name, Box::new(self.read_fixed_seq()?)),
        };

        Ok(token)
    }

    pub fn read_string(&mut self) -> Result<String> {
        let len = self.read_variant_usize()?;
        let string = self.input.read_str_uft8(len)?;
        Ok(string.to_owned())
    }

    fn read_seq<T: BytesNum + Clone>(&mut self) -> Result<Cow<'de, [T]>> {
        let num_size = size_of::<T>();
        let len = self.read_variant_usize()?;

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

    fn read_fixed_seq<const N: usize> (
        &mut self,
    ) -> Result<[Token<'de>; N]> {
        let is_field = self.state == DeState::ReadTypedValue;
        self.state = DeState::ReadUntypedValue;

        let _ = self.input.read_byte()?;
        let mut vec = vec![];

        for _ in 0..N {
            let token = self.read_any()?;
            vec.push(token);
        }

        if is_field {
            self.state = DeState::ReadTypedValue
        }

        Ok(vec.try_into().unwrap())
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
/// let bytes: [u8; 61] = [
///     0xA0, 0x01, 0x03, 0x6D, 0x73, 0x67, 0x11, 0x82, 0xA0, 0x03, 0x03, 0x70,  0x6F, 0x73, 0x41, 0x0B,
///     0x0B, 0x3F, 0xF0, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0B, 0x40, 0x00,  0x00, 0x00, 0x00, 0x00,
///     0x00, 0x00, 0x0B, 0x40, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04,  0x66, 0x6C, 0x61, 0x67,
///     0x01, 0x01, 0x05, 0x65, 0x72, 0x72, 0x6F, 0x72, 0x0A, 0x3F, 0x00, 0x00,  0x00,
/// ];
///
/// let token = decode(&bytes).unwrap();
/// ```
pub fn decode<'a>(bytes: &'a [u8]) -> Result<Token<'a>> {
    let reader = Reader::new(bytes);
    let mut deserializer: Deserializer = Deserializer::new(reader);
    deserializer.read_any()
}