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

use alloc::{borrow::Cow, boxed::Box};

use crate::core::token::Token;

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

pub fn enum_<'a>(variant_index: u8, value: Token<'a>) -> Token<'a> {
    Token::Enum(None, variant_index, Box::new(value))
}

pub fn u8_arr<'a>(values: &'a[u8]) -> Token<'a> {
    Token::U8Arr(None, values.into())
}

pub fn i32_arr<'a>(values: &'a[i32]) -> Token<'a> {
    Token::I32Arr(None, values.into())
}

pub fn i64_arr<'a>(values: &'a[i64]) -> Token<'a> {
    Token::I64Arr(None, values.into())
}

pub fn f32_arr<'a>(values: &'a[f32]) -> Token<'a> {
    Token::F32Arr(None, values.into())
}

pub fn f64_arr<'a>(values: &'a[f64]) -> Token<'a> {
    Token::F64Arr(None, values.into())
}

pub fn str_arr<'a>(values: &'a[&str]) -> Token<'a> {
    Token::StrArr(None, values.iter().map(|&s| Cow::Borrowed(s)).collect())
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