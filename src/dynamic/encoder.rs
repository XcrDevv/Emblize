use alloc::vec::Vec;
use alloc::borrow::Cow;

use crate::{
    core::{token::{Token, TokenTag}, 
    utils::endian::BytesNum}, error::Result, 
    ser::serializer::{SerState, Serializer, SerializerBuf}
};

macro_rules! try_write_name {
    ($self:expr, $name:expr) => {
        if let Some(name) = $name {
            $self.write_string(name)
        } else {
            Ok(())
        }
    };
}

impl<B: SerializerBuf> Serializer<B> { 
    pub fn write_any(&mut self, token: &Token) -> Result<()> {
        match token {
            Token::Bool(name, value)  => self.write_number(name, *value as u8,  TokenTag::from(token) as u8)?,
            Token::U8(name, value)      => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::U16(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::U32(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::U64(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::I8(name, value)      => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::I16(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::I32(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::I64(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::F32(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,
            Token::F64(name, value)    => self.write_number(name, *value,         TokenTag::from(token) as u8)?,

            Token::Str(name, value) => {
                try_write_name!(self, name)?;
                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.write_string(value)?;
            }
            Token::Enum(name, variant_index,variant) => {
                try_write_name!(self, name)?;
                self.buf.push_byte(TokenTag::from(token) as u8)?;

                if let Some(variant) = variant {
                    self.buf.push_byte(variant_index | 0x80)?;
    
                    self.write_any(variant)?;
                } else {
                    self.buf.push_byte(*variant_index)?;
                }
            }
            Token::Some(name, value) => {
                try_write_name!(self, name)?;
                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.write_any(value)?;
            },
            Token::None(name) => {
                try_write_name!(self, name)?;
                self.buf.push_byte(TokenTag::from(token) as u8)?;
            }
            Token::Struct(name, fields) => {
                try_write_name!(self, name)?;

                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.write_varint_usize(fields.len())?;

                for field in fields.iter() {
                    self.write_any(field)?;
                }
            }
            Token::EmptyArr(name) => {
                try_write_name!(self, name)?;
                self.buf.push_byte(TokenTag::from(token) as u8)?;
            },
            Token::Array(name, arr_type, values) => {
                try_write_name!(self, name)?;
                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.write_varint_usize(values.len())?;
                self.buf.push_byte(*arr_type as u8)?;
                
                let prev = self.state;
                self.state = SerState::WriteUntypedValue;
                for n in values {
                    self.write_any(n)?;
                }
                self.state = prev;
            }
            Token::Bytes(name, values) => {
                try_write_name!(self, name)?;
                self.buf.push_byte(TokenTag::from(token) as u8)?;
                self.write_varint_usize(values.len())?;
                self.buf.push_bytes(values)?;
            }

            Token::TimestampMillis(name, value)  => self.write_number(name, *value, TokenTag::from(token) as u8)?,
            Token::TimestampMicros(name, value)  => self.write_number(name, *value, TokenTag::from(token) as u8)?,
            Token::MillisSinceBoot(name, value)  => self.write_number(name, *value, TokenTag::from(token) as u8)?,
            Token::MicrosSinceBoot(name, value)  => self.write_number(name, *value, TokenTag::from(token) as u8)?,
            Token::DurationMillis(name, value)   => self.write_number(name, *value, TokenTag::from(token) as u8)?,
            Token::DurationMicros(name, value)   => self.write_number(name, *value, TokenTag::from(token) as u8)?,

            Token::Vec2(name, values) => self.write_fixed_seq(name, TokenTag::from(token), &**values)?,
            Token::Vec3(name, values) => self.write_fixed_seq(name, TokenTag::from(token), &**values)?,
            Token::Vec4(name, values) => self.write_fixed_seq(name, TokenTag::from(token), &**values)?,
        };

        Ok(())
    }

    #[inline]
    fn write_string(&mut self, string: &str) -> Result<()> {
        self.write_varint_usize(string.len())?;
        self.buf.push_bytes(string.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_number<N: BytesNum>(
        &mut self,
        name: &Option<Cow<'_, str>>,
        value: N,
        tag: u8,
    ) -> Result<()> {
        try_write_name!(self, name)?;
        if self.state != SerState::WriteUntypedValue {
            self.buf.push_byte(tag)?;
        }
        self.buf.push_bytes(value.to_be_bytes().as_ref())?;
        Ok(())
    }

    fn write_fixed_seq(
        &mut self,
        name: &Option<Cow<'_, str>>,
        tag: TokenTag,
        values: &[Token],
    ) -> Result<()> {
        try_write_name!(self, name)?;
        let vec_type = values.first()
            .map(TokenTag::from).unwrap();

        if self.state != SerState::WriteUntypedValue {
            self.buf.push_byte(tag as u8)?;
        }
        self.buf.push_byte(vec_type as u8)?;

        let prev = self.state;
        self.state = SerState::WriteUntypedValue;
        for n in values {
            self.write_any(n)?;
        }
        self.state = prev;
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
///     .u8("id", 1)
///     .vec3("data", &[1.0, 2.0, 3.0])
///     .build();
///
/// let content_bytes = encode(&data).unwrap();
/// ```
pub fn encode(tk: &Token) -> Result<Vec<u8>> {
    let mut serializer = Serializer::new();
    serializer.write_any(tk)?;
    Ok(serializer.buf)
}
