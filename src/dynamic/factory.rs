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

use crate::core::token::{Token, TokenTag};

macro_rules! impl_array_factory {
    ($fn_name:ident, $ty:ty, $variant:ident) => {
        pub fn $fn_name<'a>(values: &[$ty]) -> Token<'a> {
            let vec = values.iter().map(|&v| Token::$variant(None, v)).collect();
            Token::Array(None, TokenTag::$variant, vec)
        }
    };
}

pub fn bool<'a>(value: bool) -> Token<'a> {
    Token::Bool(None, value)
}

pub fn u8<'a>(value: u8) -> Token<'a> {
    Token::U8(None, value)
}

pub fn u16<'a>(value: u16) -> Token<'a> {
    Token::U16(None, value)
}

pub fn u32<'a>(value: u32) -> Token<'a> {
    Token::U32(None, value)
}

pub fn u64<'a>(value: u64) -> Token<'a> {
    Token::U64(None, value)
}

pub fn i8<'a>(value: i8) -> Token<'a> {
    Token::I8(None, value)
}

pub fn i16<'a>(value: i16) -> Token<'a> {
    Token::I16(None, value)
}

pub fn i32<'a>(value: i32) -> Token<'a> {
    Token::I32(None, value)
}

pub fn i64<'a>(value: i64) -> Token<'a> {
    Token::I64(None, value)
}

pub fn f32<'a>(value: f32) -> Token<'a> {
    Token::F32(None, value)
}

pub fn f64<'a>(value: f64) -> Token<'a> {
    Token::F64(None, value)
}

pub fn str<'a>(value: &'a str) -> Token<'a> {
    Token::Str(None, value.into())
}

pub fn enum_<'a>(variant_index: u8, value: Option<Token<'a>>) -> Token<'a> {
    Token::Enum(None, variant_index, value.map(Box::new))
}

pub fn option_some<'a>(value: Token<'a>) -> Token<'a> {
    Token::Some(None, Box::new(value))
}

pub fn option_none<'a>() -> Token<'a> {
    Token::None(None)
}

pub fn array<'a>(values: &'a [Token<'a>]) -> Token<'a> {
    Token::Array(None, TokenTag::Array, values.into())
}

pub fn bytes<'a>(values: &'a[u8]) -> Token<'a> {
    Token::Bytes(None, values.into())
}

impl_array_factory!(bool_array, bool, Bool);
impl_array_factory!(u8_array, u8, U8);
impl_array_factory!(u16_array, u16, U16);
impl_array_factory!(u32_array, u32, U32);
impl_array_factory!(u64_array, u64, U64);
impl_array_factory!(i8_array, i8, I8);
impl_array_factory!(i16_array, i16, I16);
impl_array_factory!(i32_array, i32, I32);
impl_array_factory!(i64_array, i64, I64);
impl_array_factory!(f32_array, f32, F32);
impl_array_factory!(f64_array, f64, F64);

pub fn str_array<'a>(values: &'a[&str]) -> Token<'a> {
    let vec = values.iter().map(|&v| Token::Str(None, v.into())).collect();
    Token::Array(None, TokenTag::Array, vec)
}

pub fn enum_array<'a>(values: &'a [Token<'a>]) -> Token<'a> {
    Token::Array(None, TokenTag::Enum, values.into())
}

pub fn option_array<'a>(values: &'a [Option<Token<'a>>]) -> Token<'a> {
    let vec = values.iter()
        .map(|v| match v {
            Some(t) => Token::Some(None, Box::new(t.clone())),
            None => Token::None(None),
        })
        .collect();
    Token::Array(None, TokenTag::Enum, vec)
}

impl_array_factory!(timestamp_ms_array, u64, TimestampMillis);
impl_array_factory!(timestamp_us_array, u64, TimestampMicros);
impl_array_factory!(ms_since_boot_array, u64, MillisSinceBoot);
impl_array_factory!(us_since_boot_array, u64, MicrosSinceBoot);
impl_array_factory!(duration_ms_array, i64, DurationMillis);
impl_array_factory!(duration_us_array, i64, DurationMicros);

impl_array_factory!(vec2_array, [f32; 2], Vec2);
impl_array_factory!(vec3_array, [f32; 3], Vec3);
impl_array_factory!(vec4_array, [f32; 4], Vec4);
impl_array_factory!(quaternion_array, [f32; 4], Quat);


pub fn struct_array<'a>(values: &'a [Token<'a>]) -> Token<'a> {
    Token::Array(None, TokenTag::Struct, values.into())
}

pub fn timestamp_ms<'a>(value: u64) -> Token<'a> {
    Token::TimestampMillis(None, value)
}

pub fn timestamp_us<'a>(value: u64) -> Token<'a> {
    Token::TimestampMicros(None, value)
}

pub fn ms_since_boot<'a>(value: u64) -> Token<'a> {
    Token::MillisSinceBoot(None, value)
}

pub fn us_since_boot<'a>(value: u64) -> Token<'a> {
    Token::MicrosSinceBoot(None, value)
}

pub fn duration_ms<'a>(value: i64) -> Token<'a> {
    Token::DurationMillis(None, value)
}

pub fn duration_us<'a>(value: i64) -> Token<'a> {
    Token::DurationMicros(None, value)
}

pub fn vec2<'a>(values: [f32; 2]) -> Token<'a> {
    Token::Vec2(None, values)
}

pub fn vec3<'a>(values: [f32; 3]) -> Token<'a> {
    Token::Vec3(None, values)
}

pub fn vec4<'a>(values: [f32; 4]) -> Token<'a> {
    Token::Vec4(None, values)
}

pub fn quaternion<'a>(values: [f32; 4]) -> Token<'a> {
    Token::Quat(None, values)
}