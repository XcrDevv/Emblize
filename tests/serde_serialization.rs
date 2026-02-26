#![cfg(feature = "alloc")]

use core::f32;
use std::vec;

use emblize::{core::{math::*, time::*, token::TokenTag}, deserialize, ser::serialize_to_alloc_vec};
use serde::Serialize;

#[test]
fn serialize_true() {
    let value = true;
    let expected = vec![TokenTag::Bool as u8, 0x01];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_false() {
    let value = false;
    let expected = vec![TokenTag::Bool as u8, 0x00];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_u8() {
    let value: u8 = 6;
    let expected = vec![TokenTag::U8 as u8, 0x06];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_u32() {
    let value: u32 = 1633837924;
    let expected = vec![TokenTag::U32 as u8, 0x61, 0x62, 0x63, 0x64];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_i8() {
    let value: i8 = -15;
    let expected = vec![TokenTag::I8 as u8, 0xF1];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_i32() {
    let value: i32 = -10329244;
    let expected = vec![TokenTag::I32 as u8, 0xFF, 0x62, 0x63, 0x64];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_f32() {
    let value: f32 = -3.009215926773463e+38;
    let expected = vec![TokenTag::F32 as u8, 0xFF, 0x62, 0x63, 0x64];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn f32_nan_roundtrip() {
    let value = f32::NAN;

    let bytes = serialize_to_alloc_vec(&value).unwrap();
    let decoded: f32 = deserialize(&bytes).unwrap();

    assert!(decoded.is_nan());
}

#[test]
fn serialize_f32_inf() {
    let value = f32::INFINITY;

    let expected = vec![TokenTag::F32 as u8, 0x7F, 0x80, 0x00, 0x00];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_f32_neg_inf() {
    let value = f32::NEG_INFINITY;

    let expected = vec![TokenTag::F32 as u8, 0xFF, 0x80, 0x00, 0x00];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    );
}

#[test]
fn serialize_f64() {
    let value: f64 = -4.035208983966375e+305;
    let expected = vec![TokenTag::F64 as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_string() {
    let value = String::from("Emblize");
    let expected = vec![TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_str() {
    let value = "Emblize";
    let expected = vec![TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_bytes() {
    let value: &[u8] = &[0x00, 0xFF, 0x33, 0x26];
    let expected = vec![TokenTag::U8Arr as u8, 0x00, 0x04, 0x00, 0xFF, 0x33, 0x26];

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

    let value = E::VariantB;
    let expected = vec![TokenTag::Enum as u8, 0x01];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_vec2() {
    let value = Vec2::new(0.0, 1.0);
    let expected = vec![TokenTag::Vec2 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
    ];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_vec3() {
    let value = Vec3::new(0.0, 1.0, 2.0);
    let expected = vec![TokenTag::Vec3 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
    ];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_vec4() {
    let value = Vec4::new(0.0, 1.0, 2.0, 3.0);
    let expected = vec![TokenTag::Vec4 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_quaternion() {
    let value = Quat::new(0.0, 1.0, 2.0, 3.0);
    let expected = vec![TokenTag::Quat as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_timestamp_ms() {
    let value = TimestampMillis(7017280452245743464);
    let expected = vec![TokenTag::TimestampMillis as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_ms_since_boot() {
    let value = MillisSinceBoot(7017280452245743464);
    let expected = vec![TokenTag::MillisSinceBoot as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_duration_ms() {
    let value = DurationMillis(-44363763471194264);
    let expected = vec![TokenTag::DurationMillis as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_empty_seq() {
    let value: Vec<u8> = vec![];
    let expected = vec![TokenTag::EmptyArr as u8];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_str_seq() {
    let value: Vec<&str> = vec!["a", "b"];
    let expected = vec![TokenTag::StrArr as u8, 0x00, 0x02, 0x00, 0x01, 0x61, 0x00, 0x01, 0x62];   

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_seq_i32() {
    let value: Vec<i32> = vec![1, 2];
    let expected = vec![TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_tuple_i32() {
    let value: (i32, i32) = (1, 2);
    let expected = vec![TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_struct() {
    #[derive(Serialize)]
    struct S {
        f: u8
    }

    let value = S { f: 0 };
    let expected = vec![TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66, TokenTag::U8 as u8, 0x00];

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

    let value = S { f: T { g: U { h: 0 } } };
    let expected = vec![
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x67,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x68,
        TokenTag::U8 as u8, 0x00,
    ];

    assert_eq!(
        serialize_to_alloc_vec(&value).unwrap(),
        expected
    )
}