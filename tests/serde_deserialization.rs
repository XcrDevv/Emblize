#![cfg(feature = "alloc")]

use core::f32;
use emblize::core::token::TokenTag;
use emblize::types::*;
use emblize::from_bytes;
// use emblize::{core::token::TokenTag, deserialize, core::math::*, core::time::*};
use serde::Deserialize;

#[test]
fn deserialize_true() {
    let value = [TokenTag::Bool as u8, 0x01];
    let expected = true;

    assert_eq!(
        from_bytes::<bool>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_false() {
    let value = [TokenTag::Bool as u8, 0x00];
    let expected = false;

    assert_eq!(
        from_bytes::<bool>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u8() {
    let value = [TokenTag::U8 as u8, 0x06];
    let expected: u8 = 6;

    assert_eq!(
        from_bytes::<u8>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u32() {
    let value = [TokenTag::U32 as u8, 0x61, 0x62, 0x63, 0x64];
    let expected: u32 =1633837924;

    assert_eq!(
        from_bytes::<u32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i8() {
    let value = [TokenTag::I8 as u8, 0xF1];
    let expected: i8 = -15;

    assert_eq!(
        from_bytes::<i8>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i32() {
    let value = [TokenTag::I32 as u8, 0xFF, 0x62, 0x63, 0x64];
    let expected: i32 = -10329244;

    assert_eq!(
        from_bytes::<i32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f32() {
    let value = [TokenTag::F32 as u8, 0xFF, 0x62, 0x63, 0x64];
    let expected: f32 =  -3.009215926773463e+38;

    assert_eq!(
        from_bytes::<f32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f32_inf() {
    let value = [TokenTag::F32 as u8, 0x7F, 0x80, 0x00, 0x00];
    let expected: f32 =  f32::INFINITY;

    assert_eq!(
        from_bytes::<f32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f32_neg_inf() {
    let value = [TokenTag::F32 as u8, 0xFF, 0x80, 0x00, 0x00];
    let expected: f32 =  f32::NEG_INFINITY;

    assert_eq!(
        from_bytes::<f32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f64() {
    let value = [TokenTag::F64 as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected: f64 = -4.035208983966375e+305;

    assert_eq!(
        from_bytes::<f64>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_string() {
    let value = [TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];
    let expected = String::from("Emblize");

    assert_eq!(
        from_bytes::<String>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_str() {
    let value = [TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];
    let expected = "Emblize";

    assert_eq!(
        from_bytes::<&str>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_bytes() {
    let value = [TokenTag::U8Arr as u8, 0x00, 0x04, 0x00, 0xFF, 0x33, 0x26];
    let expected: Vec<u8> = vec![0x00, 0xFF, 0x33, 0x26];

    assert_eq!(
        from_bytes::<Vec<u8>>(&value).unwrap(),
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

    let value = [TokenTag::Enum as u8, 0x02];
    let expected = E::VariantB;

    assert_eq!(
        from_bytes::<E>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_newtype_variant() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    enum E {
        #[allow(dead_code)]
        VariantA(u8),
        VariantB(u8),
    }

    let value = vec![TokenTag::Enum as u8, 0x01, TokenTag::U8 as u8, 0x20];
    let expected = E::VariantB(32);

    assert_eq!(
        from_bytes::<E>(&value).unwrap(),
        expected
    )
}

#[test]
fn serialize_struct_variant() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    enum E {
        #[allow(dead_code)]
        VariantA { u: u8 },
        VariantB { v: bool },
    }

    let value = vec![TokenTag::Enum as u8, 0x01, TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x76, TokenTag::Bool as u8, 0x01];
    let expected = E::VariantB { v: true };

    assert_eq!(
        from_bytes::<E>(&value).unwrap(),
        expected
    )
}

#[test]
fn deserialize_vec2() {
    let value = [TokenTag::Vec2 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
    ];
    let expected = Vec2::new(0.0, 1.0);

    assert_eq!(
        from_bytes::<Vec2>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_vec3() {
    let value = [TokenTag::Vec3 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
    ];
    let expected = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(
        from_bytes::<Vec3>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_vec4() {
    let value = [TokenTag::Vec4 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ];
    let expected = Vec4::new(0.0, 1.0, 2.0, 3.0);

    assert_eq!(
        from_bytes::<Vec4>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_quaternion() {
    let value = [TokenTag::Quat as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
        0x40, 0x40, 0x00, 0x00,
    ];
    let expected = Quat::new(0.0, 1.0, 2.0, 3.0);

    assert_eq!(
        from_bytes::<Quat>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_timestamp_ms() {
    let value = [TokenTag::TimestampMillis as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected = TimestampMillis(7017280452245743464);

    assert_eq!(
        from_bytes::<TimestampMillis>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_ms_since_boot() {
    let value = [TokenTag::MillisSinceBoot as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected = MillisSinceBoot(7017280452245743464);

    assert_eq!(
        from_bytes::<MillisSinceBoot>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_duration_ms() {
    let value = [TokenTag::DurationMillis as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected = DurationMillis(-44363763471194264);

    assert_eq!(
        from_bytes::<DurationMillis>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_empty_seq() {
    let value = [TokenTag::EmptyArr as u8];
    let expected: Vec<u8> = vec![];

    assert_eq!(
        from_bytes::<Vec<u8>>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_str_seq() {
    let value = [TokenTag::StrArr as u8, 0x00, 0x02, 0x00, 0x01, 0x61, 0x00, 0x01, 0x62];
    let expected: Vec<&str> = vec!["a", "b"];

    assert_eq!(
        from_bytes::<Vec<&str>>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_seq_i32() {
    let value = [TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let expected: Vec<i32> = vec![1, 2];

    assert_eq!(
        from_bytes::<Vec<i32>>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_tuple_i32() {
    let value = [TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let expected: (i32, i32) = (1, 2);

    assert_eq!(
        from_bytes::<(i32, i32)>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_struct() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct S {
        f: u8
    }

    let value = [TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66, TokenTag::U8 as u8, 0x00];
    let expected = S { f: 0 };

    assert_eq!(
        from_bytes::<S>(&value).unwrap(),
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

    let value =[
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x66,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x67,
        TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x68,
        TokenTag::U8 as u8, 0x00,
    ];
    let expected = S { f: T { g: U { h: 0 } } };

    assert_eq!(
        from_bytes::<S>(&value).unwrap(),
        expected
    );
}