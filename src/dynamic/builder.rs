//! Dynamic structure builder utilities.
//!
//! This module provides [`StructBuilder`], a fluent builder API for
//! constructing dynamic [`Token::Struct`] values at runtime.
//!
//! It is primarily intended for cases where the structure of the data
//! is not known at compile time, or when building token trees
//! programmatically (e.g. dynamic serialization scenarios).
//!
//! The builder allows incrementally adding fields, arrays, enums,
//! and nested structures, producing a final [`Token`] via
//! [`StructBuilder::build`].
//!
//! # Example
//!
//! ```rust
//! use emblize::dynamic::StructBuilder;
//!
//! let token = StructBuilder::new_root()
//!     .u8("flag", 1)
//!     .f32_array("values", &[3.0, 5.0])
//!     .build();
//! ```
//!
//! For creating standalone, unnamed primitive values,
//! see the helper constructors in the same module
//! (e.g. `u8(10)`, `bool(true)`).

use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::borrow::Cow;

use crate::core::token::{Token, TokenTag};

/// A builder for creating a structured [`Token::Root`] with named fields.
///
/// before finalizing it using [`build`](Builder::build).
///
/// Example:
/// ```
/// use emblize::dynamic::StructBuilder;
///
/// let token = StructBuilder::new_root()
///     .u8("id", 1)
///     .f32("x", 12.5)
///     .f32("y", -3.8)
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct StructBuilder<'a> {
    name: Option<&'a str>,
    tokens: Vec<Token<'a>>,
}

impl<'a> StructBuilder<'a> {
    /// Creates a new `StructBuilder` with the given `name`.
    ///
    /// The provided name will be associated with the resulting
    /// [`Token::Struct`] when [`build`](Self::build) is called.
    ///
    /// Use this constructor when building a named struct field
    /// inside another structure.
    pub fn new(name: &'a str) -> Self {
        Self {
            name: Some(name),
            tokens: Vec::new(),
        }
    }

    /// Creates a new root `StructBuilder` without a name.
    ///
    /// This is typically used for the top-level structure, where
    /// no field name is required. The resulting [`Token::Struct`]
    /// will contain `None` as its name when [`build`](Self::build)
    /// is called.
    pub fn new_root() -> Self {
        Self {
            name: None,
            tokens: Vec::new(),
        }
    }

    pub fn map(mut self, builder: StructBuilder<'a>) -> Self {
        let token = builder.build();
        self.tokens.push(token);
        self
    }

    pub fn empty_arr(mut self, name: &'a str) -> Self {
        self.tokens.push(Token::EmptyArr(Some(name.into())));
        self
    }

    pub fn enum_(mut self, name: &'a str, variant_index: u8, token: Option<Token<'a>>) -> Self {
        self.tokens.push(Token::Enum(Some(name.into()), variant_index, token.map(Box::new)));
        self
    }

    pub fn some(mut self, name: &'a str, token: Token<'a>) -> Self {
        self.tokens.push(Token::Some(Some(name.into()), Box::new(token)));
        self
    }

    pub fn none(mut self, name: &'a str) -> Self {
        self.tokens.push(Token::None(Some(name.into())));
        self
    }

    /// Consumes the builder and produces a [`Token::Struct`].
    ///
    /// This method finalizes the accumulated tokens and wraps them
    /// into a `Token::Struct`, preserving the optional name provided
    /// at construction time.
    ///
    /// After calling this method, the builder can no longer be used.
    pub fn build(self) -> Token<'a> {
        Token::Struct(self.name.map(Cow::Borrowed), self.tokens)
    }
}

/// Implements fluent builder methods for common token field types.
///
/// Each generated method appends a field of the given type to the structure:
///
/// - `u8("flag", 1)` → adds a [`Token::U8`] named `"flag"`
/// - `f32_arr("pos", vec![1.0, 2.0])` → adds a [`Token::F32Arr`] named `"pos"`
///
/// These methods consume and return `self` to allow chaining.
macro_rules! builder_methods {
    (
        $(
            $fn_name:ident : $variant:ident ( $ty:ty )
        ),* $(,)?
    ) => {
        impl<'a> StructBuilder<'a> {
            $(
                pub fn $fn_name(mut self, name: &'a str, value: $ty) -> Self {
                    self.tokens.push(Token::$variant(Some(name.into()), value.into()));
                    self
                }
            )*
        }
    };
}

/// Implements fluent builder methods for array token field types.
///
/// Each generated method appends an array field of the given element type
/// to the structure:
///
/// - `u8_array("flags", &[1, 2, 3])` → adds a [`Token::Array`] of [`Token::U8`] named `"flags"`
/// - `f32_array("values", &[1.0, 2.0])` → adds a [`Token::Array`] of [`Token::F32`] named `"values"`
///
/// Only `Copy` types are supported by this macro. For non-`Copy` types
/// such as strings, options, enums, and structs, see the manually
/// implemented methods [`str_arr`](StructBuilder::str_arr),
/// [`option_arr`](StructBuilder::option_arr),
/// [`enum_arr`](StructBuilder::enum_arr), and
/// [`struct_arr`](StructBuilder::struct_arr).
///
/// These methods consume and return `self` to allow chaining.
macro_rules! builder_array_methods {
    (
        $(
            $fn_name:ident : $variant:ident ( $ty:ty )
        ),* $(,)?
    ) => {
        impl<'a> StructBuilder<'a> {
            $(
                pub fn $fn_name(mut self, name: &'a str, values: &[$ty]) -> Self {
                    let vec = values.iter().map(|&v| Token::$variant(None, v.into())).collect();
                    self.tokens.push(Token::Array(Some(name.into()), TokenTag::$variant, vec));
                    self
                }
            )*
        }
    };
}

builder_methods! {
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

    string: Str(&'a str),

    bytes: Bytes(&'a [u8]),

    timestamp_ms: TimestampMillis(u64),
    timestamp_us: TimestampMicros(u64),
    ms_since_boot: MillisSinceBoot(u64),
    us_since_boot: MicrosSinceBoot(u64),
    duration_ms: DurationMillis(i64),
    duration_us: DurationMicros(i64),

    vec2: Vec2([f32; 2]),
    vec3: Vec3([f32; 3]),
    vec4: Vec4([f32; 4]),
    quaternion: Quat([f32; 4]),
}

builder_array_methods! {
    bool_array: Bool(bool),
    u8_array: U8(u8),
    u16_array: U16(u16),
    u32_array: U32(u32),
    u64_array: U64(u64),
    i8_array: I8(i8),
    i16_array: I16(i16),
    i32_array: I32(i32),
    i64_array: I64(i64),
    f32_array: F32(f32),
    f64_array: F64(f64),
    timestamp_ms_array: TimestampMillis(u64),
    timestamp_us_array: TimestampMicros(u64),
    ms_since_boot_array: MillisSinceBoot(u64),
    us_since_boot_array: MicrosSinceBoot(u64),
    duration_ms_array: DurationMillis(i64),
    duration_us_array: DurationMicros(i64),
    vec2_array: Vec2([f32; 2]),
    vec3_array: Vec3([f32; 3]),
    vec4_array: Vec4([f32; 4]),
    quaternion_array: Quat([f32; 4]),
}

impl<'a> StructBuilder<'a> {
    pub fn str_array(mut self, name: &'a str, values: &[&'a str]) -> Self {
        let vec = values.iter().map(|&v| Token::Str(None, v.into())).collect();
        self.tokens.push(Token::Array(Some(name.into()), TokenTag::Str, vec));
        self
    }

    pub fn enum_array(mut self, name: &'a str, values: &[Token<'a>]) -> Self {
        self.tokens.push(Token::Array(Some(name.into()), TokenTag::Enum, values.to_vec()));
        self
    }

    pub fn option_array(mut self, name: &'a str, values: &[Option<Token<'a>>]) -> Self {
        let vec = values.iter()
            .map(|v| match v {
                Some(t) => Token::Some(None, Box::new(t.clone())),
                None => Token::None(None),
            })
            .collect();
        self.tokens.push(Token::Array(Some(name.into()), TokenTag::Enum, vec));
        self
    }

    pub fn struct_array(mut self, name: &'a str, values: &[Token<'a>]) -> Self {
        self.tokens.push(Token::Array(Some(name.into()), TokenTag::Struct, values.to_vec()));
        self
    }
}