use std::io::{BufWriter, Cursor, Write};

use crate::{macros::BytesNum, token::Token};

impl Token {
    pub fn write(&self, writer: &mut BufWriter<&mut Cursor<Vec<u8>>>,) -> std::io::Result<()> {
        match self {
            Token::Root(tokens) => {
                writer.write_all(&[
                    0xA0,
                    tokens.len() as u8
                ])?;

                for token in tokens {
                    token.write(writer)?;
                }
            },
            Token::Struct(name, tokens) => {
                Self::write_string(writer, name)?;
                
                writer.write_all(&[0x01, tokens.len() as u8])?;

                for token in tokens {
                    token.write(writer)?;
                }
            },
            Token::U8(name, value) => Self::write_number(writer, 0x02, name, *value)?,
            Token::U16(name, value) => Self::write_number(writer, 0x03, name, *value)?,
            Token::U32(name, value) => Self::write_number(writer, 0x04, name, *value)?,
            Token::U64(name, value) => Self::write_number(writer, 0x05, name, *value)?,
            Token::I32(name, value) => Self::write_number(writer, 0x06, name, *value)?,
            Token::I64(name, value) => Self::write_number(writer, 0x07, name, *value)?,
            Token::F32(name, value) => Self::write_number(writer, 0x08, name, *value)?,
            Token::F64(name, value) => Self::write_number(writer, 0x09, name, *value)?,
            Token::Str(name, value) => {
                Self::write_string(writer, name)?;
                writer.write_all(&[0x0A])?;
                Self::write_string(writer, &value)?;
            },
            Token::U8Arr(name, values) => Self::write_number_array(writer, 0x0B, name, values)?,
            Token::I32Arr(name, values) => Self::write_number_array(writer, 0x0C, name, values)?,
            Token::I64Arr(name, values) => Self::write_number_array(writer, 0x0D, name, values)?,
            Token::F32Arr(name, values) => Self::write_number_array(writer, 0x0E, name, values)?,
            Token::F64Arr(name, values) => Self::write_number_array(writer, 0x0F, name, values)?,
        }

        Ok(())
    }

    fn write_string(writer: &mut BufWriter<&mut Cursor<Vec<u8>>>, string: &str) -> std::io::Result<()> {
        let length = string.len() as u32;
        writer.write_all(&length.to_be_bytes())?;
        writer.write_all(string.as_bytes())?;
        Ok(())
    }

    fn write_number<T: BytesNum>(writer: &mut BufWriter<&mut Cursor<Vec<u8>>>, variant: u8, name: &str, value: T) -> std::io::Result<()> {
        Self::write_string(writer, &name)?;
        writer.write_all(&[variant])?;
        writer.write_all(value.to_be_bytes().as_slice())?;
        Ok(())
    }

    fn write_number_array<T: BytesNum>(writer: &mut BufWriter<&mut Cursor<Vec<u8>>>, variant: u8, name: &str, values: &Vec<T>) -> std::io::Result<()> {
        Self::write_string(writer, &name)?;
        writer.write_all(&[variant])?;
        writer.write_all(&(values.len() as u32).to_be_bytes())?;
        let bytes: Vec<u8> = values.iter()
            .flat_map(|v| v.to_be_bytes())
            .collect();
        writer.write_all(&bytes)?;
        Ok(())
    }
}