#![cfg(feature = "alloc")]

use emblize::{core::{math::*, time::*, token::TokenTag}, ser::{serialize_to_alloc_vec}};
use serde::Serialize;

const STRUCT_PREFIX: &[u8] = &[TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x76];

fn make_expected(suffix: &[u8]) -> Vec<u8> {
    let mut v = Vec::from(STRUCT_PREFIX);
    v.extend_from_slice(suffix);
    v
}

#[test]
fn serialize_true() {
    #[derive(Serialize)]
    struct Root {
        v: bool
    }

    let value = Root { v: true };
    let expected = make_expected(&[TokenTag::Bool as u8, 0x01]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_false() {
    #[derive(Serialize)]
    struct Root {
        v: bool
    }
    
    let value = Root { v: false };
    let expected = make_expected(&[TokenTag::Bool as u8, 0x00]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_u8() {
    #[derive(Serialize)]
    struct Root {
        v: u8
    }
    
    let value = Root { v: 6 };
    let expected = make_expected(&[TokenTag::U8 as u8, 0x06]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_u16() {
    #[derive(Serialize)]
    struct Root {
        v: u16
    }

    let value = Root { v: 24930 };
    let expected = make_expected(&[TokenTag::U16 as u8, 0x61, 0x62]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_u32() {
    #[derive(Serialize)]
    struct Root {
        v: u32
    }

    let value = Root { v: 1633837924 };
    let expected = make_expected(&[TokenTag::U32 as u8, 0x61, 0x62, 0x63, 0x64]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_u64() {
    #[derive(Serialize)]
    struct Root {
        v: u64
    }

    let value = Root { v: 7017280452245743464 };
    let expected = make_expected(&[TokenTag::U64 as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_i8() {
    #[derive(Serialize)]
    struct Root {
        v: i8
    }

    let value = Root { v: -15 };
    let expected = make_expected(&[TokenTag::I8 as u8, 0xF1]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_i16() {
    #[derive(Serialize)]
    struct Root {
        v: i16
    }

    let value = Root { v: -158 };
    let expected = make_expected(&[TokenTag::I16 as u8, 0xFF, 0x62]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_i32() {
    #[derive(Serialize)]
    struct Root {
        v: i32
    }

    let value = Root { v: -10329244 };
    let expected = make_expected(&[TokenTag::I32 as u8, 0xFF, 0x62, 0x63, 0x64]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_i64() {
    #[derive(Serialize)]
    struct Root {
        v: i64
    }

    let value = Root { v: -44363763471194264 };
    let expected = make_expected(&[TokenTag::I64 as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_f32() {
    #[derive(Serialize)]
    struct Root {
        v: f32
    }

    let value = Root { v: -3.009215926773463e+38 };
    let expected = make_expected(&[TokenTag::F32 as u8, 0xFF, 0x62, 0x63, 0x64]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_f64() {
    #[derive(Serialize)]
    struct Root {
        v: f64
    }

    let value = Root { v: -4.035208983966375e+305 };
    let expected = make_expected(&[TokenTag::F64 as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_string() {
    #[derive(Serialize)]
    struct Root {
        v: String
    }

    let value = Root { v: "Emblize".into() };
    let expected = make_expected(&[TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_bytes() {
    #[derive(Serialize)]
    struct Root<'a> {
        v: &'a [u8]
    }

    let value = Root { v: &[0x00, 0xFF, 0x33, 0x26] };
    let expected = make_expected(&[TokenTag::U8Arr as u8, 0x00, 0x04, 0x00, 0xFF, 0x33, 0x26]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_unit_variant() {
    #[derive(Serialize)]
    enum E {
        #[allow(dead_code)]
        VariantA,
        VariantB,
    }

    #[derive(Serialize)]
    struct Root {
        v: E
    }

    let value = Root { v: E::VariantB };
    let expected = make_expected(&[TokenTag::Enum as u8, 0x00, 0x08, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6e, 0x74, 0x42]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_vec2() {
    #[derive(Serialize)]
    struct Root {
        v: Vec2
    }

    let value = Root { v: Vec2::new(0.0, 1.0) };
    let expected = make_expected(&[TokenTag::Vec2 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
    ]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_vec3() {
    #[derive(Serialize)]
    struct Root {
        v: Vec3
    }

    let value = Root { v: Vec3::new(0.0, 1.0, 2.0) };
    let expected = make_expected(&[TokenTag::Vec3 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
    ]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_vec4() {
    #[derive(Serialize)]
    struct Root {
        v: Vec4
    }

    let value = Root { v: Vec4::new(0.0, 1.0, 2.0, 3.0) };
    let expected = make_expected(&[TokenTag::Vec4 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_quaternion() {
    #[derive(Serialize)]
    struct Root {
        v: Quat
    }

    let value = Root { v: Quat::new(0.0, 1.0, 2.0, 3.0) };
    let expected = make_expected(&[TokenTag::Quat as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_timestamp_ms() {
    #[derive(Serialize)]
    struct Root {
        v: TimestampMillis
    }

    let value = Root { v: TimestampMillis(7017280452245743464) };
    let expected = make_expected(&[TokenTag::TimestampMillis as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_timestamp_us() {
    #[derive(Serialize)]
    struct Root {
        v: TimestampMicros
    }

    let value = Root { v: TimestampMicros(7017280452245743464) };
    let expected = make_expected(&[TokenTag::TimestampMicros as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_ms_since_boot() {
    #[derive(Serialize)]
    struct Root {
        v: MillisSinceBoot
    }

    let value = Root { v: MillisSinceBoot(7017280452245743464) };
    let expected = make_expected(&[TokenTag::MillisSinceBoot as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_us_since_boot() {
    #[derive(Serialize)]
    struct Root {
        v: MicrosSinceBoot
    }

    let value = Root { v: MicrosSinceBoot(7017280452245743464) };
    let expected = make_expected(&[TokenTag::MicrosSinceBoot as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_duration_ms() {
    #[derive(Serialize)]
    struct Root {
        v: DurationMillis
    }

    let value = Root { v: DurationMillis(-44363763471194264) };
    let expected = make_expected(&[TokenTag::DurationMillis as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_duration_us() {
    #[derive(Serialize)]
    struct Root {
        v: DurationMicros
    }

    let value = Root { v: DurationMicros(-44363763471194264) };
    let expected = make_expected(&[TokenTag::DurationMicros as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_empty_seq() {
    #[derive(Serialize)]
    struct Root {
        v: Vec<u8>
    }

    let value = Root { v: vec![] };
    let expected = make_expected(&[TokenTag::EmptyArr as u8]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_str_seq() {
    #[derive(Serialize)]
    struct Root {
        v: Vec<String>
    }

    let value = Root { v: vec!["a".into(), "b".into()] };
    let expected = make_expected(&[TokenTag::StrArr as u8, 0x00, 0x02, 0x00, 0x01, 0x61, 0x00, 0x01, 0x62]);    

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_seq_i32() {
    #[derive(Serialize)]
    struct Root {
        v: Vec<i32>
    }

    let value = Root { v: vec![1, 2] };
    let expected = make_expected(&[TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_tuple_i32() {
    #[derive(Serialize)]
    struct Root {
        v: (i32, i32)
    }

    let value = Root { v: (1, 2) };
    let expected = make_expected(&[TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
#[should_panic]
fn serialize_tuple_t() {
    #[derive(Serialize)]
    struct Root {
        v: (i32, u32)
    }

    let value = Root { v: (1, 2) };
    let _ = serialize_to_alloc_vec(&value).unwrap();
}

#[test]
fn serialize_struct() {
    #[derive(Serialize)]
    struct S {
        f: u8
    }

    #[derive(Serialize)]
    struct Root {
        v: S
    }

    let value = Root { v: S { f: 0 } };
    let expected = make_expected(&[TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66, TokenTag::U8 as u8, 0x00]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_nested_struct() {
    #[derive(Serialize)]
    struct S {
        f: T
    }

    #[derive(Serialize)]
    struct T {
        g: U,
    }

    #[derive(Serialize)]
    struct U {
        h: u8
    }

    #[derive(Serialize)]
    struct Root {
        v: S
    }

    let value = Root { v: S { f: T { g: U { h: 0 } } } };
    let expected = make_expected(&[
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x67,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x68,
        TokenTag::U8 as u8, 0x00,
    ]);

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}