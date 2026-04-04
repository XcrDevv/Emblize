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
fn deserialize_string() {
    let value = [TokenTag::Str as u8, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];
    let expected = String::from("Emblize");

    assert_eq!(
        from_bytes::<String>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_str() {
    let value = [TokenTag::Str as u8, 0x07, 0x45, 0x6D, 0x62, 0x6C, 0x69, 0x7A, 0x65];
    let expected = "Emblize";

    assert_eq!(
        from_bytes::<&str>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_bytes() {
    let value = [TokenTag::Bytes as u8, 0x04, 0x00, 0xFF, 0x33, 0x26];
    let expected: Vec<u8> = vec![0x00, 0xFF, 0x33, 0x26];

    assert_eq!(
        from_bytes::<&[u8]>(&value).unwrap(),
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

    let value = vec![TokenTag::Enum as u8, 0x81, TokenTag::U8 as u8, 0x20];
    let expected = E::VariantB(32);

    assert_eq!(
        from_bytes::<E>(&value).unwrap(),
        expected
    )
}

#[test]
fn deserialize_struct_variant() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    enum E {
        #[allow(dead_code)]
        VariantA { u: u8 },
        VariantB { v: bool },
    }

    let value = vec![TokenTag::Enum as u8, 0x01, TokenTag::Struct as u8, 0x01, 0x01, 0x76, TokenTag::Bool as u8, 0x01];
    let expected = E::VariantB { v: true };

    assert_eq!(
        from_bytes::<E>(&value).unwrap(),
        expected
    )
}

#[test]
fn deserialize_some() {
    let value = vec![TokenTag::Some as u8, TokenTag::Bool as u8, 0x01];
    let expected = Some(true);

    assert_eq!(
        from_bytes::<Option<bool>>(&value).unwrap(),
        expected
    )
}

#[test]
fn deserialize_some_nested() {
    let value = vec![TokenTag::Some as u8, TokenTag::Some as u8, TokenTag::Bool as u8, 0x01];
    let expected = Some(Some(true));

    assert_eq!(
        from_bytes::<Option<Option<bool>>>(&value).unwrap(),
        expected
    )
}

#[test]
fn deserialize_none() {
    let value = vec![TokenTag::None as u8];
    let expected = None;

    assert_eq!(
        from_bytes::<Option<bool>>(&value).unwrap(),
        expected
    )
}


#[test]
fn deserialize_vec3() {
    let value = [TokenTag::Vec3 as u8, TokenTag::F32 as u8,
        0x00, 0x00, 0x00, 0x00, 
        0x3F, 0x80, 0x00, 0x00,
        0x40, 0x00, 0x00, 0x00,
    ];
    let expected = Vec3::new(0.0, 1.0, 2.0);

    assert_eq!(
        from_bytes::<Vec3<f32>>(&value).unwrap(),
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
fn deserialize_empty_seq() {
    let value = [TokenTag::EmptyArr as u8];
    let expected: Vec<u8> = vec![];

    assert_eq!(
        from_bytes::<Vec<u8>>(&value).unwrap(),
        expected
    );
}

#[test]
fn deserialize_array() {
    let value = vec![TokenTag::Array as u8, 0x02, TokenTag::I32 as u8, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let expected: Vec<i32> = vec![1, 2];

    assert_eq!(
        from_bytes::<Vec<i32>>(&value).unwrap(),
        expected
    )
}


#[test]
fn deserialize_struct() {
    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct S {
        f: u8
    }

    let value = [TokenTag::Struct as u8, 0x01, 0x01, 0x66, TokenTag::U8 as u8, 0x00];
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
        TokenTag::Struct as u8, 0x01, 0x01, 0x66,
        TokenTag::Struct as u8, 0x01, 0x01, 0x67,
        TokenTag::Struct as u8, 0x01, 0x01, 0x68,
        TokenTag::U8 as u8, 0x00,
    ];
    let expected = S { f: T { g: U { h: 0 } } };

    assert_eq!(
        from_bytes::<S>(&value).unwrap(),
        expected
    );
}