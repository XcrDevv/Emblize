use alloc::vec::Vec;
use alloc::borrow::Cow;

use crate::{
    core::{token::{Token, TokenTag}, 
    utils::endian::BytesNum}, error::Result, 
    ser::serializer::{Serializer, SerializerBuf}
};


impl<B: SerializerBuf> Serializer<B> { 
    pub fn write_any(&mut self, token: &Token) -> Result<()> {
        self.found_token = TokenTag::from(token) as u8;

        match token {
            Token::Bool(name, value) => self.write_number(name, *value as u8)?,
            Token::U8(name, value) => self.write_number(name, *value)?,
            Token::U16(name, value) => self.write_number(name, *value)?,
            Token::U32(name, value) => self.write_number(name, *value)?,
            Token::U64(name, value) => self.write_number(name, *value)?,
            Token::I8(name, value) => self.write_number(name, *value)?,
            Token::I16(name, value) => self.write_number(name, *value)?,
            Token::I32(name, value) => self.write_number(name, *value)?,
            Token::I64(name, value) => self.write_number(name, *value)?,
            Token::F32(name, value) => self.write_number(name, *value)?,
            Token::F64(name, value) => self.write_number(name, *value)?,

            Token::Str(name, value) => {
                if let Some(name) = name {
                    self.write_string(name)?;
                }
                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.write_string(value)?;
            }
            Token::Enum(name, variant_index,variant) => {
                if let Some(name) = name {
                    self.write_string(name)?;
                }
                self.buf.push_byte(TokenTag::from(token) as u8)?;

                if let Some(variant) = variant {
                    self.buf.push_byte(variant_index | 0x80)?;
    
                    self.write_any(variant)?;
                } else {
                    self.buf.push_byte(*variant_index)?;
                }
            }
            Token::Option(name, value) => {
                if let Some(name) = name {
                    self.write_string(name)?;
                }
                self.buf.push_byte(TokenTag::from(token) as u8)?;

                if let Some(value) = value {
                    self.buf.push_byte(0x01)?;
                    self.write_any(value)?;
                } else {
                    self.buf.push_byte(0x00)?;
                }
            }

            Token::Struct(name, fields) => {
                if let Some(name) = name {
                    self.write_string(name)?;
                }

                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.buf.push_byte(fields.len() as u8)?;


                for field in fields.iter() {
                    self.write_any(field)?;
                }
            }
            Token::EmptyArr(name) => {
                if let Some(name) = name {
                    self.write_string(name)?;
                }
                self.buf.push_byte(TokenTag::from(token) as u8)?;
            },
            Token::U8Arr(name, values) => self.write_seq(name, values)?,
            Token::I32Arr(name, values) => self.write_seq(name, values)?,
            Token::I64Arr(name, values) => self.write_seq(name, values)?,
            Token::F32Arr(name, values) => self.write_seq(name, values)?,
            Token::F64Arr(name, values) => self.write_seq(name, values)?,
            Token::StrArr(name, values) => {
                if let Some(name) = name {
                    self.write_string(name)?;
                }
                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.buf.push_bytes(&(values.len() as u16).to_be_bytes())?;
                for value in values.iter() {
                    self.write_string(value)?;
                }
            }

            Token::TimestampMillis(name, value) => self.write_number(name, *value)?,
            Token::TimestampMicros(name, value) => self.write_number(name, *value)?,
            Token::MillisSinceBoot(name, value) => self.write_number(name, *value)?,
            Token::MicrosSinceBoot(name, value) => self.write_number(name, *value)?,
            Token::DurationMillis(name, value) => self.write_number(name, *value)?,
            Token::DurationMicros(name, value) => self.write_number(name, *value)?,

            Token::Vec2(name, values) => self.write_fixed_seq(name, values)?,
            Token::Vec3(name, values) => self.write_fixed_seq(name, values)?,
            Token::Vec4(name, values) => self.write_fixed_seq(name, values)?,
            Token::Quat(name, values) => self.write_fixed_seq(name, values)?,
        };

        Ok(())
    }

    fn write_string(&mut self, string: &str) -> Result<()> {
        let length = string.len() as u16;
        self.buf.push_bytes(&length.to_be_bytes())?;
        self.buf.push_bytes(string.as_bytes())?;
        Ok(())
    }

    fn write_number<N: BytesNum>(&mut self, name: &Option<Cow<'_, str>>, value: N) -> Result<()>
    {
        if let Some(name) = name {
            self.write_string(&name)?;
        }
        self.buf.push_byte(self.found_token)?;
        self.buf.push_bytes(value.to_be_bytes().as_ref())?;
        Ok(())
    }

    fn write_seq<N: BytesNum>(&mut self, name: &Option<Cow<'_, str>>, values: &[N]) -> Result<()> {
        if let Some(name) = name {
            self.write_string(&name)?;
        }
        self.buf.push_byte(self.found_token)?;
        self.buf.push_bytes(&(values.len() as u16).to_be_bytes())?;
        for n in values {
            self.buf.push_bytes(n.to_be_bytes().as_ref())?;
        }
        Ok(())
    }

    fn write_fixed_seq<N: BytesNum>(&mut self, name: &Option<Cow<'_, str>>, values: &[N]) -> Result<()> {
        if let Some(name) = name {
            self.write_string(&name)?;
        }
        self.buf.push_byte(self.found_token)?;
        for n in values {
            self.buf.push_bytes(n.to_be_bytes().as_ref())?;
        }
        Ok(())
    }
}

/// Serializes a [`Token`] into its binary representation.
///
/// This function encodes the provided token into the format
/// expected by the corresponding [`decode`] function and
/// returns the resulting bytes.
///
/// # Errors
///
/// Returns an [`Error`] if the token cannot be written to the
/// internal buffer (for example, due to an unsupported value
/// or serialization failure).
///
/// # Examples
///
/// ```rust
/// use emblize::dynamic::{StructBuilder, encode};
///
/// let data = StructBuilder::new_root()
///     .u8("flag", 1)
///     .f32_arr("data", &[3.0, 5.0])
///     .build();
///
/// let content_bytes = encode(&data).unwrap();
/// ```
pub fn encode(tk: &Token) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();
    serializer.write_any(tk)?;
    Ok(serializer.buf)
}
