#![cfg(feature = "alloc")]

use emblize::dynamic::{encode, decode, StructBuilder, factory::*};
use emblize::core::token::Token;
use emblize::types::*;

#[test]
fn build_tokens() {
    let token = StructBuilder::new_root()
        .f32("f32", 1.0)
        .string("str", "lorem")
        .variant("enm", 1, Some(bool(true)))
        .map("struct", |b| {
            b.f32("f32", 3.14)
        })
        .bytes("u8_arr", &[1, 2, 3])
        .timestamp_ms("tsms", 1)
        .vec3::<f64>("vec3", &Vec3::default().as_arr())
        .build();

    let expected_token = Token::Struct(None, vec![
        Token::F32(Some("f32".into()), 1.0),
        Token::Str(Some("str".into()), "lorem".into()),
        Token::Enum(Some("enm".into()), 1, Some(Box::new(bool(true)))),
        Token::Struct(Some("struct".into()), vec![
            Token::F32(Some("f32".into()), 3.14),
        ]),
        Token::Bytes(Some("u8_arr".into()), vec![1, 2, 3].into()),
        Token::TimestampMillis(Some("tsms".into()), 1),
        Token::Vec3(Some("vec3".into()), Box::new([Token::F64(None, 0.0), Token::F64(None, 0.0), Token::F64(None, 0.0)])),
    ]);

    assert_eq!(token, expected_token)
}

#[test]
fn serialize_deserialize_false() {
    let token = StructBuilder::new_root()
        .bool("f", false)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Bool(Some("f".into()), false),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_true() {
    let token = StructBuilder::new_root()
        .bool("t", true)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Bool(Some("t".into()), true),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_f32() {
    let token = StructBuilder::new_root()
        .f32("f32", -3.009215926773463e+38)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::F32(Some("f32".into()), -3.009215926773463e+38),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_string() {
    let token = StructBuilder::new_root()
        .string("str", "Emblize")
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Str(Some("str".into()), "Emblize".into()),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_enum() {
    let token = StructBuilder::new_root()
        .variant("enm", 1, Some(bool(true)))
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Enum(Some("enm".into()), 1, Some(Box::new(bool(true)))),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_some() {
    let token = StructBuilder::new_root()
        .some("opt", u8(6))
        .build();

    let expected_token = Token::Struct(None, vec![
        Token::Some(Some("opt".into()), Box::new(u8(6)))
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_struct() {
    let token = StructBuilder::new_root()
        .map("struct", |b| {
            b.f32("f32", 3.14)
        })
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Struct(Some("struct".into()), vec![
            Token::F32(Some("f32".into()), 3.14),
        ]),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_empty_arr() {
    let token = StructBuilder::new_root()
        .empty_arr("e_arr")
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::EmptyArr(Some("e_arr".into())),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_bytes() {
    let token = StructBuilder::new_root()
        .bytes("bytes", &[0x00, 0xFF, 0x33, 0x26])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Bytes(Some("bytes".into()), vec![0x00, 0xFF, 0x33, 0x26].into()),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_array() {
    let token = StructBuilder::new_root()
        .bytes("array", &[0x00, 0xFF, 0x33, 0x26])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Bytes(Some("array".into()), vec![0x00, 0xFF, 0x33, 0x26].into()),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_timestamp_ms() {
    let token = StructBuilder::new_root()
        .timestamp_ms("tsms", 7017280452245743464)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::TimestampMillis(Some("tsms".into()), 7017280452245743464),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_vec3() {
    let token = StructBuilder::new_root()
        .vec3("vec3", &Vec3::new(0.0, 1.0, 2.0).as_arr())
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Vec3(Some("vec3".into()), Box::new([Token::F64(None, 0.0), Token::F64(None, 1.0), Token::F64(None, 2.0)])),
    ]);

    let bytes = encode(&token).unwrap();
    let result = decode(&bytes).unwrap();

    assert_eq!(result, expected_token)
}