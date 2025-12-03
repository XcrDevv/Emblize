// Each variant of `Token` is represented by one byte.
// Except for the `End` token, all variant includes a name:
// A string preceded by 4 bytes indicating its length,
// followed by `n` bytes corresponding to the characters in the string
#[derive(Debug, PartialEq)]
pub enum Token {
    Struct(String, Vec<Token>), // 0x01
    U8(String, u8),             // 0x02
    U16(String, u16),           // 0x03
    U32(String, u32),           // 0x04
    U64(String, u64),           // 0x05
    I32(String, i32),           // 0x06
    I64(String, i64),           // 0x07
    F32(String, f32),           // 0x08
    F64(String, f64),           // 0x09
    Str(String, String),        // 0x0A
    U8Arr(String, Vec<u8>),     // 0x0B
    I32Arr(String, Vec<i32>),   // 0x0C
    I64Arr(String, Vec<i64>),   // 0x0D
    F32Arr(String, Vec<f32>),   // 0x0E
    F64Arr(String, Vec<f64>),   // 0x0F

    Root(Vec<Token>),           // 0xA0
}