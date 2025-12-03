use std::io::{BufReader, Cursor, Error, Read};

use crate::{macros::BytesNum, token::Token};

impl Token {
    pub fn read_root(reader: &mut BufReader<Cursor<&[u8]>>) -> std::io::Result<Token> {
        let token_variant = Self::read_byte(reader)?;

        if token_variant != 0xA0 {
            return Err(Error::new(std::io::ErrorKind::InvalidData, "Root token expected at start"));
        }

        let field_count = Self::read_byte(reader)?;
        let mut tokens = Vec::with_capacity(field_count as usize);

        for _ in 0..field_count {
            let token = Self::read(reader)?;
            tokens.push(token);
        }

        Ok(Token::Root(tokens))
    }

    pub fn read(reader: &mut BufReader<Cursor<&[u8]>>) -> std::io::Result<Token> {
        let name = Self::read_string(reader)?;

        let token_variant = Self::read_byte(reader)?;

        println!("{}", name);

        match token_variant {
            0x01 => {
                let field_count = Self::read_byte(reader)?;
                let mut tokens = Vec::with_capacity(field_count as usize);
                for _ in 0..field_count {
                    let token = Self::read(reader)?;
                    tokens.push(token);
                }
                Ok(Token::Struct(name, tokens))
            }
            0x02 => Ok(Token::U8(name, Self::read_byte(reader)?)),
            0x03 => Ok(Token::U16(name, Self::read_number(reader)?)),
            0x04 => Ok(Token::U32(name, Self::read_number(reader)?)),
            0x05 => Ok(Token::U64(name, Self::read_number(reader)?)),
            0x06 => Ok(Token::I32(name, Self::read_number(reader)?)),
            0x07 => Ok(Token::I64(name, Self::read_number(reader)?)),
            0x08 => Ok(Token::F32(name, Self::read_number(reader)?)),
            0x09 => Ok(Token::F64(name, Self::read_number(reader)?)),
            0x0A => Ok(Token::Str(name, Self::read_string(reader)?)),
            0x0B => Ok(Token::U8Arr(name, Self::read_number_array(reader)?)),
            0x0C => Ok(Token::I32Arr(name, Self::read_number_array(reader)?)),
            0x0D => Ok(Token::I64Arr(name, Self::read_number_array(reader)?)),
            0x0E => Ok(Token::F32Arr(name, Self::read_number_array(reader)?)),
            0x0F => Ok(Token::F64Arr(name, Self::read_number_array(reader)?)),
            _ => Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Unknown token"))
        }
    }

    fn read_byte(reader: &mut BufReader<Cursor<&[u8]>>) -> Result<u8, std::io::Error> {
        let mut buf = [0; 1];
        reader.read_exact(&mut buf)?;
        Ok(buf[0])
    }

    fn read_string(reader: &mut BufReader<Cursor<&[u8]>>) -> Result<String, std::io::Error> {
        let mut size_buf = [0; 4];
        reader.read_exact(&mut size_buf)?;
        let size = u32::from_be_bytes(size_buf) as usize;
        let mut string_buf = vec![0; size];
        reader.read_exact(&mut string_buf)?;
        Ok(String::from_utf8(string_buf).unwrap())
    }

    fn read_number<T: BytesNum>(reader: &mut BufReader<Cursor<&[u8]>>) -> Result<T, std::io::Error> {
        let mut buf = vec![0; size_of::<T>()];
        reader.read_exact(&mut buf)?;
        let value = T::from_be_bytes(buf.as_slice().try_into().unwrap());
        Ok(value)
    }

    fn read_number_array<T: BytesNum>(reader: &mut BufReader<Cursor<&[u8]>>) -> Result<Vec<T>, std::io::Error> {
        let mut len_buf = [0; 4];
        reader.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;
        let mut values_buf = vec![0; len as usize * size_of::<T>()];
        reader.read_exact(&mut values_buf)?;
        let values: Vec<T> = values_buf.chunks_exact(size_of::<T>())
            .map(|chunk| T::from_be_bytes(chunk.try_into().unwrap()))
            .collect();
        Ok(values)
    }
} 