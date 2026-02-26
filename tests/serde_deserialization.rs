#![cfg(feature = "alloc")]

use core::f32;

use emblize::{core::token::TokenTag, deserialize, core::math::*, core::time::*};
use serde::Deserialize;

#[test]
fn deserialize_true() {
    let value = [TokenTag::Bool as u8, 0x01];
    let expected = true;

    assert_eq!(
        deserialize::<bool>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_false() {
    let value = [TokenTag::Bool as u8, 0x00];
    let expected = false;

    assert_eq!(
        deserialize::<bool>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u8() {
    let value = [TokenTag::U8 as u8, 0x06];
    let expected: u8 = 6;

    assert_eq!(
        deserialize::<u8>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_u32() {
    let value = [TokenTag::U32 as u8, 0x61, 0x62, 0x63, 0x64];
    let expected: u32 =1633837924;

    assert_eq!(
        deserialize::<u32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i8() {
    let value = [TokenTag::I8 as u8, 0xF1];
    let expected: i8 = -15;

    assert_eq!(
        deserialize::<i8>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_i32() {
    let value = [TokenTag::I32 as u8, 0xFF, 0x62, 0x63, 0x64];
    let expected: i32 = -10329244;

    assert_eq!(
        deserialize::<i32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f32() {
    let value = [TokenTag::F32 as u8, 0xFF, 0x62, 0x63, 0x64];
    let expected: f32 =  -3.009215926773463e+38;

    assert_eq!(
        deserialize::<f32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f32_inf() {
    let value = [TokenTag::F32 as u8, 0x7F, 0x80, 0x00, 0x00];
    let expected: f32 =  f32::INFINITY;

    assert_eq!(
        deserialize::<f32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f32_neg_inf() {
    let value = [TokenTag::F32 as u8, 0xFF, 0x80, 0x00, 0x00];
    let expected: f32 =  f32::NEG_INFINITY;

    assert_eq!(
        deserialize::<f32>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_f64() {
    let value = [TokenTag::F64 as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected: f64 = -4.035208983966375e+305;

    assert_eq!(
        deserialize::<f64>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_string() {
    let value = [TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];
    let expected = String::from("Emblize");

    assert_eq!(
        deserialize::<String>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_str() {
    let value = [TokenTag::Str as u8, 0x00, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];
    let expected = "Emblize";

    assert_eq!(
        deserialize::<&str>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_bytes() {
    let value = [TokenTag::U8Arr as u8, 0x00, 0x04, 0x00, 0xFF, 0x33, 0x26];
    let expected: Vec<u8> = vec![0x00, 0xFF, 0x33, 0x26];

    assert_eq!(
        deserialize::<Vec<u8>>(&value).unwrap(),
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

    let value = [TokenTag::Enum as u8, 0x00, 0x08, 0x56, 0x61, 0x72, 0x69, 0x61, 0x6e, 0x74, 0x42];
    let expected = E::VariantB;

    assert_eq!(
        deserialize::<E>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_vec2() {
    let value = [TokenTag::Vec2 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
    ];
    let expected = Vec2::new(0.0, 1.0);

    assert_eq!(
        deserialize::<Vec2>(&value).unwrap(),
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
        deserialize::<Vec3>(&value).unwrap(),
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
        deserialize::<Vec4>(&value).unwrap(),
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
        deserialize::<Quat>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_timestamp_ms() {
    let value = [TokenTag::TimestampMillis as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected = TimestampMillis(7017280452245743464);

    assert_eq!(
        deserialize::<TimestampMillis>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_ms_since_boot() {
    let value = [TokenTag::MillisSinceBoot as u8, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected = MillisSinceBoot(7017280452245743464);

    assert_eq!(
        deserialize::<MillisSinceBoot>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_duration_ms() {
    let value = [TokenTag::DurationMillis as u8, 0xFF, 0x62, 0x63, 0x64, 0x65, 0x66, 0x67, 0x68];
    let expected = DurationMillis(-44363763471194264);

    assert_eq!(
        deserialize::<DurationMillis>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_empty_seq() {
    let value = [TokenTag::EmptyArr as u8];
    let expected: Vec<u8> = vec![];

    assert_eq!(
        deserialize::<Vec<u8>>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_str_seq() {
    let value = [TokenTag::StrArr as u8, 0x00, 0x02, 0x00, 0x01, 0x61, 0x00, 0x01, 0x62];
    let expected: Vec<&str> = vec!["a", "b"];

    assert_eq!(
        deserialize::<Vec<&str>>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_seq_i32() {
    let value = [TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let expected: Vec<i32> = vec![1, 2];

    assert_eq!(
        deserialize::<Vec<i32>>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_tuple_i32() {
    let value = [TokenTag::I32Arr as u8, 0x00, 0x02, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let expected: (i32, i32) = (1, 2);

    assert_eq!(
        deserialize::<(i32, i32)>(&value).unwrap(),
        expected
    );
}

#[test]
#[should_panic]
fn deserialize_tuple_t() {
    let value = [TokenTag::I32Arr as u8, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let _ = deserialize::<(i32, u32)>(&value).unwrap();
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
        deserialize::<S>(&value).unwrap(),
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
        deserialize::<S>(&value).unwrap(),
        expected
    );
}