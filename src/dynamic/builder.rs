use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::borrow::Cow;

use crate::core::token::Token;

/// A builder for creating a structured [`Token::Root`] with named fields.
///
/// before finalizing it using [`build`](Builder::build).
///
/// Example:
/// ```
/// use emblize::*;
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
    /// Creates a new root `Builder`
    pub fn new(name: &'a str) -> Self {
        Self {
            name: Some(name),
            tokens: Vec::new(),
        }
    }

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

    pub fn enum_(mut self, name: &'a str, variant_index: u8, token: Token<'a>) -> Self {
        self.tokens.push(Token::Enum(Some(name.into()), variant_index, Box::new(token)));
        self
    }

    pub fn string_arr(mut self, name: &'a str, values: &'a [&'a str]) -> Self {
        self.tokens.push(Token::StrArr(
            Some(name.into()), 
            values.iter().map(|&s| Cow::Borrowed(s)).collect()
        ));
        self
    }

    /// Finalizes the root structure and returns it as a [`Token::Root`] value.
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

    u8_arr: U8Arr(&'a [u8]),
    i32_arr: I32Arr(&'a [i32]),
    i64_arr: I64Arr(&'a [i64]),
    f32_arr: F32Arr(&'a [f32]),
    f64_arr: F64Arr(&'a [f64]),

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
