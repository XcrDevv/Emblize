#[cfg(feature = "alloc")]
use alloc::{
    borrow::Cow, boxed::Box, vec::Vec
};
use num_enum::TryFromPrimitive;
use crate::error::Error;

#[repr(u8)]
#[derive(Debug, TryFromPrimitive, PartialEq, Eq, Clone, Copy)]
pub enum TokenTag {
    Bool        = 0x01,
    U8          = 0x02,
    U16         = 0x03,
    U32         = 0x04,
    U64         = 0x05,
    I8          = 0x06,
    I16         = 0x07,
    I32         = 0x08,
    I64         = 0x09,
    F32         = 0x0A,
    F64         = 0x0B,

    Str         = 0x10,
    Enum        = 0x11,
    Some        = 0x12,
    None        = 0x13,

    EmptyArr    = 0x21,
    U8Arr       = 0x22,
    I32Arr      = 0x23,
    I64Arr      = 0x24,
    F32Arr      = 0x25,
    F64Arr      = 0x26,
    StrArr      = 0x27,

    TimestampMillis     = 0x30,
    TimestampMicros     = 0x31,
    MillisSinceBoot     = 0x32,
    MicrosSinceBoot     = 0x33,
    DurationMillis      = 0x34,
    DurationMicros      = 0x35,

    Vec2 = 0x40,
    Vec3 = 0x41,
    Vec4 = 0x42,
    Quat = 0x43,

    Struct = 0xA0,
}

macro_rules! impl_from_to_token_str {
    ($($variant:ident),*) => {
        impl TryFrom<&'static str> for TokenTag {
            type Error = Error;

            fn try_from(value: &'static str) -> core::result::Result<Self, Error> {
                match value {
                    $(
                        stringify!($variant) => Ok(TokenTag::$variant),
                    )*
                    _ => Err(Error::InvalidToken)
                }
            }
        }

        impl From<TokenTag> for &'static str {
            fn from(value: TokenTag) -> Self {
                match value {
                    $(
                        TokenTag::$variant => stringify!($variant),
                    )*
                }
            }
        }
    }
}

impl_from_to_token_str!(
    Bool, U8, U16, U32, U64, I8, I16, I32, I64, F32, F64, 
    Str, Enum, Some, None,
    Struct, EmptyArr, U8Arr, I32Arr, I64Arr, F32Arr, F64Arr, StrArr, 
    TimestampMillis, TimestampMicros, MillisSinceBoot, MicrosSinceBoot, DurationMillis, DurationMicros, 
    Vec2, Vec3, Vec4, Quat
);

#[cfg(feature = "alloc")]
type Name<'a> = Option<Cow<'a, str>>;

/// A serialized token in the binary format.
///
/// Each token starts with a one-byte discriminant indicating its variant.
///
/// Most variants carry a `Name<'a>` identifying the field they belong to.
/// The `Struct` variant is the exception: it does not have a `Name`, but
/// optionally carries its own type name.
///
/// Depending on the variant, the payload may contain:
/// - A primitive value (integer, float, bool)
/// - A string or array (owned or borrowed)
/// - A nested token (`Enum`)
/// - A collection of nested tokens (`Struct`)
/// - Time-related values (timestamps, durations, etc.)
/// - Math types (Vec2, Vec3, Vec4, Quat)
///
/// The exact binary layout of the payload depends on the variant.
#[cfg(feature = "alloc")]
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Bool(Name<'a>, bool),
    U8(Name<'a>, u8),
    U16(Name<'a>, u16),
    U32(Name<'a>, u32),
    U64(Name<'a>, u64),
    I8(Name<'a>, i8),
    I16(Name<'a>, i16),
    I32(Name<'a>, i32),
    I64(Name<'a>, i64),
    F32(Name<'a>, f32),
    F64(Name<'a>, f64),

    Str(Name<'a>, Cow<'a, str>),
    Enum(Name<'a>, u8, Option<Box<Token<'a>>>),
    Some(Name<'a>, Box<Token<'a>>),
    None(Name<'a>),

    EmptyArr(Name<'a>),
    U8Arr(Name<'a>, Cow<'a, [u8]>),
    I32Arr(Name<'a>, Cow<'a, [i32]>),
    I64Arr(Name<'a>, Cow<'a, [i64]>),
    F32Arr(Name<'a>, Cow<'a, [f32]>),
    F64Arr(Name<'a>, Cow<'a, [f64]>),
    StrArr(Name<'a>, Cow<'a, [Cow<'a, str>]>),

    TimestampMillis(Name<'a>, u64),
    TimestampMicros(Name<'a>, u64),
    MillisSinceBoot(Name<'a>, u64),
    MicrosSinceBoot(Name<'a>, u64),
    DurationMillis(Name<'a>, i64),
    DurationMicros(Name<'a>, i64),

    Vec2(Name<'a>, [f32; 2]),
    Vec3(Name<'a>, [f32; 3]),
    Vec4(Name<'a>, [f32; 4]),
    Quat(Name<'a>, [f32; 4]),

    Struct(Option<Cow<'a, str>>, Vec<Token<'a>>),
}

#[cfg(feature = "alloc")]
impl<'a> Token<'a> {
    pub fn name(&'a self) -> &'a str {
        match self {
            Token::Bool(name, _)
            | Token::U8(name, _)
            | Token::U16(name, _)
            | Token::U32(name, _)
            | Token::U64(name, _)
            | Token::I8(name, _)
            | Token::I16(name, _)
            | Token::I32(name, _)
            | Token::I64(name, _)
            | Token::F32(name, _)
            | Token::F64(name, _)
            | Token::Str(name, _)
            | Token::Enum(name, _, _)
            | Token::EmptyArr(name)
            | Token::Some(name, _)
            | Token::None(name)
            | Token::U8Arr(name, _)
            | Token::I32Arr(name, _)
            | Token::I64Arr(name, _)
            | Token::F32Arr(name, _)
            | Token::F64Arr(name, _)
            | Token::StrArr(name, _)
            | Token::TimestampMillis(name, _)
            | Token::TimestampMicros(name, _)
            | Token::MillisSinceBoot(name, _)
            | Token::MicrosSinceBoot(name, _)
            | Token::DurationMillis(name, _)
            | Token::DurationMicros(name, _)
            | Token::Vec2(name, _)
            | Token::Vec3(name, _)
            | Token::Vec4(name, _)
            | Token::Quat(name, _) => {
                name.as_deref().expect("Token must have a name")
            }

            Token::Struct(name, _) => {
                name.as_deref().expect("Struct must have a name")
            }
        }
    }
}

/// Generates the implementation of the `matches` method for `TokenTag`.
///
/// This macro creates a method that validates whether a given byte corresponds to the expected
/// token type. If the types match, it returns `Ok(())`. If they don't match, it returns a
/// specific error indicating the type that was found.
///
/// # Arguments
///
/// - A comma-separated list of `TokenTag` variant identifiers
///
/// # Generates
///
/// ```rust,ignore
/// impl TokenTag {
///     pub fn matches(&self, tag: u8) -> Result<(), Error> {
///         // Converts the byte to TokenTag or returns error if unknown
///         // Compares with self and returns Ok if they match
///         // If they don't match, returns specific error for the found type
///     }
/// }
/// ```
///
/// # Usage Example
///
/// ```rust,ignore
/// let expected = TokenTag::U32;
/// let byte_tag = 0x04; // Corresponds to U32
/// 
/// // This returns Ok(())
/// expected.matches(byte_tag)?;
/// 
/// let wrong_byte = 0x08; // Corresponds to I32
/// // This returns Err(Error::ExpectedType("I32"))
/// expected.matches(wrong_byte)?;
/// ```
macro_rules! impl_matches_token_tag {
    ($($variant:ident),*) => {
        impl TokenTag {
            pub fn matches(&self, tag: u8) -> Result<(), Error> {
                let tag = TokenTag::try_from(tag).map_err(|_| Error::UnknownDType(tag))?;

                if *self == tag {
                    return Ok(());
                }

                match tag {
                    $(
                        TokenTag::$variant => Err(Error::ExpectedType(stringify!($variant))),
                    )*
                }
            }
        }
    };
}

/// Generates the conversion implementation from `&Token` to `TokenTag`.
///
/// This macro creates the `From` trait implementation that allows obtaining the corresponding
/// tag for any `Token` variant. It uses pattern matching to extract the token type regardless
/// of its internal fields.
///
/// # Arguments
///
/// - A comma-separated list of variant identifiers that must exist in both
///   `Token` and `TokenTag`
///
/// # Generates
///
/// ```rust,ignore
/// impl<'a> From<&Token<'a>> for TokenTag {
///     fn from(token: &Token<'a>) -> Self {
///         match token {
///             Token::U32(..) => TokenTag::U32,
///             Token::Str(..) => TokenTag::Str,
///             // ... for all variants
///         }
///     }
/// }
/// ```
///
/// # Usage Example
///
/// ```rust,ignore
/// let token = Token::U32(Cow::Borrowed("age"), 25);
/// let tag: TokenTag = (&token).into();
/// 
/// assert_eq!(tag, TokenTag::U32);
/// ```
#[cfg(feature = "alloc")]
macro_rules! impl_from_token_to_tag {
    ($($variant:ident),*) => {
        impl<'a> From<&Token<'a>> for TokenTag {
            fn from(token: &Token<'a>) -> Self {
                match token {
                    $(
                        Token::$variant(..) => TokenTag::$variant,
                    )*
                }
            }
        }
    };
}

// Generates the `matches` implementation for all token types
impl_matches_token_tag!(
    Bool, U8, U16, U32, U64, I8, I16, I32, I64, F32, F64, 
    Str, Enum, Some, None,
    Struct, EmptyArr, U8Arr, I32Arr, I64Arr, F32Arr, F64Arr, StrArr, 
    TimestampMillis, TimestampMicros, MillisSinceBoot, MicrosSinceBoot, DurationMillis, DurationMicros, 
    Vec2, Vec3, Vec4, Quat
);

// Generates the `Token` -> `TokenTag` conversion implementation for all types
#[cfg(feature = "alloc")]
impl_from_token_to_tag!(
    Bool, U8, U16, U32, U64, I8, I16, I32, I64, F32, F64, 
    Str, Enum, Some, None,
    Struct, EmptyArr, U8Arr, I32Arr, I64Arr, F32Arr, F64Arr, StrArr, 
    TimestampMillis, TimestampMicros, MillisSinceBoot, MicrosSinceBoot, DurationMillis, DurationMicros, 
    Vec2, Vec3, Vec4, Quat
);