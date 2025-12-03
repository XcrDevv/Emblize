use crate::Token;

/// A builder for creating a structured [`Token::Root`] with named fields.
///
/// before finalizing it using [`build`](Builder::build).
///
/// Example:
/// ```
/// use emblize::*;
///
/// let token = Builder::new()
///     .u8("id", 1)
///     .f32("x", 12.5)
///     .f32("y", -3.8)
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct Builder {
    tokens: Vec<Token>
}

impl Builder {
    /// Creates a new root `Builder`
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn map(mut self, builder: StructBuilder) -> Self {
        let token = builder.build();
        self.tokens.push(token);
        self
    }

    /// Finalizes the root structure and returns it as a [`Token::Root`] value.
    pub fn build(self) -> Token {
        Token::Root(self.tokens)
    }
}

/// Builder for creating a structured [`Token::Struct`] with named fields.
///
/// Example:
/// ```
/// use emblize::*;
///
/// let token = Builder::new()
///     .map(
///         StructBuilder::new("Object")
///             .u8("id", 1)
///     )
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct StructBuilder {
    name: String,
    tokens: Vec<Token>
}

impl StructBuilder {
    /// Creates a new `StructBuilder`
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            tokens: vec![]
        }
    }

    pub fn map(mut self, builder: StructBuilder) -> Self {
        let token = builder.build();
        self.tokens.push(token);
        self
    }

    /// Finalizes the root structure and returns it as a [`Token::Struct`] value.
    pub fn build(self) -> Token {
        Token::Struct(self.name, self.tokens)
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
        impl Builder {
            $(
                pub fn $fn_name(mut self, name: &str, value: $ty) -> Self {
                    self.tokens.push(Token::$variant(name.into(), value));
                    self
                }
            )*
        }

        impl StructBuilder {
            $(
                pub fn $fn_name(mut self, name: &str, value: $ty) -> Self {
                    self.tokens.push(Token::$variant(name.into(), value));
                    self
                }
            )*
        }
    };
}

builder_methods! {
    u8: U8(u8),
    u16: U16(u16),
    u32: U32(u32),
    u64: U64(u64),
    i32: I32(i32),
    i64: I64(i64),
    f32: F32(f32),
    f64: F64(f64),
    string: Str(String),
    u8_arr: U8Arr(Vec<u8>),
    i32_arr: I32Arr(Vec<i32>),
    i64_arr: I64Arr(Vec<i64>),
    f32_arr: F32Arr(Vec<f32>),
    f64_arr: F64Arr(Vec<f64>),
}