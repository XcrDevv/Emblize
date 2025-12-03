pub mod macros;
pub mod token;
pub mod read;
pub mod write;
pub mod builder;

#[cfg(feature = "serde_feature")]
pub mod error;
#[cfg(feature = "serde_feature")]
pub mod deserializer;
#[cfg(feature = "serde_feature")]
pub mod serializer;

use std::io::{BufReader, BufWriter, Cursor};
pub use token::Token;
pub use builder::{Builder, StructBuilder};

#[cfg(feature = "serde_feature")]
pub use serializer::Serializer;

/// Serializes a `Token` into a binary format as a vector of bytes.
/// # Errors
/// Returns an error if the token cannot be written to an internal buffer.
/// # Example
/// ```
/// use emblize::*;
/// 
/// let data = Builder::new()
///     .u8("flag", 1)
///     .f32_arr("pos", vec![3.0, 5.0])
///     .build();
/// 
/// let content_bytes = as_bytes(&data);
/// ```
pub fn as_bytes(tk: &Token) -> std::io::Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());

    {
        let mut writer = BufWriter::new(&mut buffer);
        tk.write(&mut writer)?;
    }

    Ok(buffer.into_inner())
}

/// Reads a byte buffer and converts it into a `Token`.
/// 
/// The bytes must follow the expected binary format.
/// 
/// # Errors
/// Returns an [`std::io::Error`] if reading or parsing the bytes fails.
/// # Example
/// ```
/// use emblize::*;
/// 
/// let bytes = std::fs::read("./output/data.dat").unwrap();
/// let data_readed = from_bytes(&bytes).unwrap();
/// ```
pub fn from_bytes(bytes: &[u8]) -> std::io::Result<Token> {
    let mut reader = BufReader::new(Cursor::new(bytes));
    Ok(Token::read_root(&mut reader)?)
}

#[cfg(test)]
mod tests {
    use crate::builder::StructBuilder;

    use super::*;

    #[test]
    fn test_root_builder() {
        let token = Builder::new()
            .u8("u8", 1)
            .u16("u16", 1)
            .u32("u32", 1)
            .u64("u64", 1)
            .string("string", "lorem".into())
            .u8_arr("u8_arr", vec![1, 2, 3])
            .f32_arr("f32_arr", vec![1.0, 2.0, 3.0])
            .build();

        let expected_token = Token::Root(vec![
            Token::U8("u8".into(), 1),
            Token::U16("u16".into(), 1),
            Token::U32("u32".into(), 1),
            Token::U64("u64".into(), 1),
            Token::Str("string".into(), "lorem".into()),
            Token::U8Arr("u8_arr".into(), vec![1, 2, 3]),
            Token::F32Arr("f32_arr".into(), vec![1.0, 2.0, 3.0]),
        ]);

        assert_eq!(token, expected_token)
    }

    #[test]
    fn test_struct_builder() {
        let map = StructBuilder::new("map")
            .i32("id", 1)
            .map(
                StructBuilder::new("obj")
                .u8("value", 10)
            )
            .build();

        let expected_map = Token::Struct("map".into(), vec![
            Token::I32("id".into(), 1),
            Token::Struct("obj".into(), vec![
                Token::U8("value".into(), 10)
            ])
        ]);

        assert_eq!(map, expected_map)
    }

    #[test]
    fn test_from_bytes() {
        let bytes = [
            0xA0, 0x03, 0x00, 0x00, 0x00, 0x01, 0x78, 0x08,
            0x3F, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x79, 0x08, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x7A, 0x08, 0x40, 0x40, 0x00, 0x00,
        ];
        
        let expected_token = Token::Root(vec![
            Token::F32("x".into(), 1.0),
            Token::F32("y".into(), 2.0),
            Token::F32("z".into(), 3.0),
        ]);

        assert_eq!(
            from_bytes(&bytes).expect("Failed to read from bytes"), 
            expected_token
        )
    }

    #[test]
    fn test_as_bytes() {
        let token = Token::Root(vec![
            Token::F32("x".into(), 1.0),
            Token::F32("y".into(), 2.0),
            Token::F32("z".into(), 3.0),
        ]);

        let expected_bytes = [
            0xA0, 0x03, 0x00, 0x00, 0x00, 0x01, 0x78, 0x08,
            0x3F, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
            0x79, 0x08, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x01, 0x7A, 0x08, 0x40, 0x40, 0x00, 0x00,
        ];

        assert_eq!(
            as_bytes(&token).expect("Failed to convert to bytes"),
            expected_bytes
        )
    }
}