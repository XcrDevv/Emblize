#![cfg(feature = "alloc")]

use emblize::{core::token::TokenTag, deserialize, core::math::*, core::time::*};
use serde::Deserialize;

const STRUCT_PREFIX: &[u8] = &[TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x76];

fn make_value(suffix: &[u8]) -> Vec<u8> {
    let mut v = Vec::from(STRUCT_PREFIX);
    v.extend_from_slice(suffix);
    v
}

#[test]
fn deserialize_true() {
   #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: bool
    }

    let value = make_value(&[TokenTag::Bool as u8, 0x01]);
    let expected = Root { v: true };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_false() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: bool
    }

    let value = make_value(&[TokenTag::Bool as u8, 0x00]);
    let expected = Root { v: false };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u8() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: u8
    }

    let value = make_value(&[TokenTag::U8 as u8, 0x06]);
    let expected = Root { v: 6 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u16() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: u16
    }

    let value = make_value(&[TokenTag::U16 as u8, 0x61, 0x62]);
    let expected = Root { v: 24930 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u32() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: u32
    }

    let value = make_value(&[TokenTag::U32 as u8, 0x61, 0x62, 0x63, 0x64]);
    let expected = Root { v: 1633837924 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u64() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: u64
    }

    let value = make_value(&[TokenTag::U64 as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: 7017280452245743464 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i8() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: i8
    }

    let value = make_value(&[TokenTag::I8 as u8, 0xF1]);
    let expected = Root { v: -15 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i16() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: i16
    }

    let value = make_value(&[TokenTag::I16 as u8, 0xFF, 0x62]);
    let expected = Root { v: -158 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i32() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: i32
    }

    let value = make_value(&[TokenTag::I32 as u8, 0xFF, 0x62, 0x63, 0x64]);
    let expected = Root { v: -10329244 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i64() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: i64
    }

    let value = make_value(&[TokenTag::I64 as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: -44363763471194264 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f32() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Root {
        v: f32
    }

    let value = make_value(&[TokenTag::F32 as u8, 0xFF, 0x62, 0x63, 0x64]);
    let expected = Root { v: -3.009215926773463e+38 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f64() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Root {
        v: f64
    }

    let value = make_value(&[TokenTag::F64 as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: -4.035208983966375e+305 };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_string() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: String
    }

    let value = make_value(&[TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65]);
    let expected = Root { v: "Emblize".into() };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_str() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root<'a> {
        v: &'a str
    }

    let value = make_value(&[TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65]);
    let expected = Root { v: "Emblize" };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_bytes() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: Vec<u8>
    }

    let value = make_value(&[TokenTag::U8Arr as u8, 0x00, 0x04, 0x00, 0xFF, 0x33, 0x26]);
    let expected = Root { v: vec![0x00, 0xFF, 0x33, 0x26] };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_unit_variant() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    enum E {
        #[allow(dead_code)]
        VariantC,
        #[allow(dead_code)]
        VariantA,
        VariantB,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: E
    }

    let value = make_value(&[TokenTag::Enum as u8, 0x00, 0x08, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6e, 0x74, 0x42]);
    let expected = Root { v: E::VariantB };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_vec2() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Root {
        v: Vec2
    }

    let value = make_value(&[TokenTag::Vec2 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
    ]);
    let expected = Root { v: Vec2::new(0.0, 1.0) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_vec3() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Root {
        v: Vec3
    }

    let value = make_value(&[TokenTag::Vec3 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
    ]);
    let expected = Root { v: Vec3::new(0.0, 1.0, 2.0) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_vec4() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Root {
        v: Vec4
    }

    let value = make_value(&[TokenTag::Vec4 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ]);
    let expected = Root { v: Vec4::new(0.0, 1.0, 2.0, 3.0) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_quaternion() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Root {
        v: Quat
    }

    let value = make_value(&[TokenTag::Quat as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ]);
    let expected = Root { v: Quat::new(0.0, 1.0, 2.0, 3.0) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_timestamp_ms() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: TimestampMillis
    }

    let value = make_value(&[TokenTag::TimestampMillis as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: TimestampMillis(7017280452245743464) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_timestamp_us() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: TimestampMicros
    }

    let value = make_value(&[TokenTag::TimestampMicros as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: TimestampMicros(7017280452245743464) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_ms_since_boot() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: MillisSinceBoot
    }

    let value = make_value(&[TokenTag::MillisSinceBoot as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: MillisSinceBoot(7017280452245743464) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_us_since_boot() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: MicrosSinceBoot
    }

    let value = make_value(&[TokenTag::MicrosSinceBoot as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: MicrosSinceBoot(7017280452245743464) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_duration_ms() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: DurationMillis
    }

    let value = make_value(&[TokenTag::DurationMillis as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: DurationMillis(-44363763471194264) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_duration_us() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: DurationMicros
    }

    let value = make_value(&[TokenTag::DurationMicros as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);
    let expected = Root { v: DurationMicros(-44363763471194264) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_empty_seq() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: Vec<u8>
    }

    let value = make_value(&[TokenTag::EmptyArr as u8]);
    let expected = Root { v: vec![] };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_str_seq() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: Vec<String>
    }

    let value = make_value(&[TokenTag::StrArr as u8, 0x00, 0x02, 0x00, 0x01, 0x61, 0x00, 0x01, 0x62]);
    let expected = Root { v: vec!["a".into(), "b".into()] };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_seq_i32() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: Vec<i32>
    }

    let value = make_value(&[TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]);
    let expected = Root { v: vec![1, 2] };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_tuple_i32() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: (i32, i32)
    }

    let value = make_value(&[TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]);
    let expected = Root { v: (1, 2) };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
#[should_panic]
fn deserialize_tuple_t() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Root {
        v: (i32, u32)
    }

    let value = make_value(&[TokenTag::I32Arr as u8, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]);
    let _ = deserialize::<Root>(&value).unwrap();
}


#[test]
fn deserialize_struct() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct S {
        f: u8
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: S
    }

    let value = make_value(&[TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66, TokenTag::U8 as u8, 0x00]);
    let expected = Root { v: S { f: 0 } };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_nested_struct() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct S {
        f: T
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct T {
        g: U,
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct U {
        h: u8
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Root {
        v: S
    }

    let value = make_value(&[
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x67,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x68,
        TokenTag::U8 as u8, 0x00,
    ]);
    let expected = Root { v: S { f: T { g: U { h: 0 } } } };

    assert_eq!(
        deserialize::<Root>(&value).unwrap(),
        expected
    );
}