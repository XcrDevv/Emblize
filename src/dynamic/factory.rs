//! Helper constructors for fieldless [`Token`] values.
//!
//! This module provides small convenience functions for creating
//! primitive [`Token`] variants without an associated field name.
//!
//! Instead of manually constructing tokens like:
//!
//! ```rust
//! # use emblize::core::token::Token;
//! let tk = Token::U32(None, 10);
//! ```
//!
//! You can use the more ergonomic helpers:
//!
//! ```rust
//! # use emblize::dynamic::factory::*;
//! let tk = u32(10);
//! ```
//!
//! All functions in this module create *unnamed* (`None`) tokens,
//! which makes them especially useful when building standalone
//! values, array elements, or enum payloads.

use alloc::boxed::Box;

use crate::{core::token::{Token, TokenTag}, error::{Error, Result}, types::VectorNumber};

macro_rules! type_factory {
    (
        $(
            $fn_name:ident : $variant:ident ( $ty:ty )
        ),* $(,)?
    ) => {
        $(
            pub fn $fn_name<'a>(value: $ty) -> Token<'a> {
                Token::$variant(None, value)
            }
        )*
    };
}

type_factory! {
    bool: Bool(bool),
    u8: U8(u8),
    u16: U16(u16),
    u32: U32(u32),
    u64: U64(u64),
    i8: I8(i8),
    i16: I16(i16),
    i32: I32(i32),
    i64: I64(i64),
    f32: F32(f32),
    f64: F64(f64),

    timestamp_ms: TimestampMillis(u64),
    timestamp_us: TimestampMicros(u64),
    ms_since_boot: MillisSinceBoot(u64),
    us_since_boot: MicrosSinceBoot(u64),
    duration_ms: DurationMillis(i64),
    duration_us: DurationMicros(i64),

    // vec2: Vec2([f32; 2]),
    // vec3: Vec3([f32; 3]),
    // vec4: Vec4([f32; 4]),
    // quaternion: Quat([f32; 4]),
}

pub fn str<'a>(value: &'a str) -> Token<'a> {
    Token::Str(None, value.into())
}

pub fn variant<'a>(variant_index: u8, value: Option<Token<'a>>) -> Token<'a> {
    Token::Enum(None, variant_index, value.map(Box::new))
}

pub fn option_some<'a>(value: Token<'a>) -> Token<'a> {
    Token::Some(None, Box::new(value))
}

pub fn option_none<'a>() -> Token<'a> {
    Token::None(None)
}

/// Creates a typed array token from a list of tokens, returning an `Error` if the elements are heterogeneous (not the same type).
pub fn array<'a, I>(values: I) -> Result<Token<'a>>
where
    I: IntoIterator<Item = Token<'a>>,
{

    let mut iter = values.into_iter();

    let first_token = match iter.next() {
        Some(token) => token,
        None => {
            return Ok(Token::EmptyArr(None))
        }
    };

    let first_tag = TokenTag::from(&first_token);

    let mut collected = Vec::new();
    collected.push(first_token);

    for token in iter {
        let tag = TokenTag::from(&token);
        if tag != first_tag {
            return Err(Error::HeterogeneousTuple {
                expected: first_tag as u8,
                got: tag as u8,
            });
        }
        collected.push(token);
    }

    Ok(Token::Array(None, first_tag, collected))
}

/// Creates a typed array token without checking for type homogeneity.
/// 
/// # Safety
/// All tokens in `values` must be of the same type.
pub unsafe fn array_unchecked<'a>(values: &'a [Token<'a>]) -> Token<'a> {
    if values.is_empty() {
        return Token::EmptyArr(None)
    }

    let array_type = TokenTag::from(&values[0]);
    Token::Array(None, array_type, values.into())  
}

pub fn bytes<'a>(values: &'a[u8]) -> Token<'a> {
    Token::Bytes(None, values.into())
}

pub fn vec2<'a, T: VectorNumber + Into<Token<'a>>>(values: &'a [T; 2]) -> Token<'a> {
    let tokens: Vec<Token> = values.iter().map(|&v| v.into()).collect();
    Token::Vec2(None, Box::new(tokens.try_into().unwrap()))
}

pub fn vec3<'a, T: VectorNumber + Into<Token<'a>>>(values: &'a [T; 3]) -> Token<'a> {
    let tokens: Vec<Token> = values.iter().map(|&v| v.into()).collect();
    Token::Vec3(None, Box::new(tokens.try_into().unwrap()))
}

pub fn vec4<'a, T: VectorNumber + Into<Token<'a>>>(values: &'a [T; 4]) -> Token<'a> {
    let tokens: Vec<Token> = values.iter().map(|&v| v.into()).collect();
    Token::Vec4(None, Box::new(tokens.try_into().unwrap()))
}

pub fn quaternion<'a, T: VectorNumber + Into<Token<'a>>>(values: &'a [T; 4]) -> Token<'a> {
    let tokens: Vec<Token> = values.iter().map(|&v| v.into()).collect();
    Token::Quat(None, Box::new(tokens.try_into().unwrap()))
}

macro_rules! impl_from_number {
    (
        $(
            $variant:ident : $ty:ty
        ),* $(,)?
    ) => {
        $(
            impl<'a> From<$ty> for Token<'a> {
                fn from(value: $ty) -> Self {
                    Token::$variant(None, value)
                }
            }
        )*
    };
}

impl_from_number! {
    Bool: bool,
    U8: u8,
    U16: u16,
    U32: u32,
    U64: u64,
    I8: i8,
    I16: i16,
    I32: i32,
    I64: i64,
    F32: f32,
    F64: f64,

}