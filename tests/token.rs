#![cfg(feature = "alloc")]

use emblize::{StructBuilder, as_bytes, core::{math::*, token::Token}, from_bytes, platform::factory::*};

#[test]
fn build_tokens() {
    let token = StructBuilder::new_root()
        .bool("f", false)
        .bool("t", true)
        .u8("u8", 1)
        .u16("u16", 1)
        .u32("u32", 1)
        .u64("u64", 1)
        .i8("i8", 1)
        .i16("i16", 1)
        .i32("i32", 1)
        .i64("i64", 1)
        .f32("f32", 1.0)
        .f64("f64", 1.0)
        .string("str", "lorem")
        .enum_("enm", 1, bool(true))
        .map(StructBuilder::new("struct").f32("f32", 3.14))
        .map(StructBuilder::new("nested").f32("f32", 2.7182))
        .empty_arr("e_arr")
        .u8_arr("u8_arr", &[1, 2, 3])
        .i32_arr("i32_arr", &[1, 2, 3])
        .i64_arr("i64_arr", &[1, 2, 3])
        .f32_arr("f32_arr", &[1.0, 2.0, 3.0])
        .f64_arr("f64_arr", &[1.0, 2.0, 3.0])
        .string_arr("string_arr", &["i", "j", "k"])
        .timestamp_ms("tsms", 1)
        .timestamp_us("tsus", 1)
        .ms_since_boot("mssb", 1)
        .us_since_boot("ussb", 1)
        .duration_ms("dnms", 1)
        .duration_us("dnus", 1)
        .vec2("vec2", Vec2::default().as_arr())
        .vec3("vec3", Vec3::default().as_arr())
        .vec4("vec4", Vec4::default().as_arr())
        .quaternion("quat", Quat::default().as_arr())
        .build();

    let expected_token = Token::Struct(None, vec![
        Token::Bool(Some("f".into()), false),
        Token::Bool(Some("t".into()), true),
        Token::U8(Some("u8".into()), 1),
        Token::U16(Some("u16".into()), 1),
        Token::U32(Some("u32".into()), 1),
        Token::U64(Some("u64".into()), 1),
        Token::I8(Some("i8".into()), 1),
        Token::I16(Some("i16".into()), 1),
        Token::I32(Some("i32".into()), 1),
        Token::I64(Some("i64".into()), 1),
        Token::F32(Some("f32".into()), 1.0),
        Token::F64(Some("f64".into()), 1.0),
        Token::Str(Some("str".into()), "lorem".into()),
        Token::Enum(Some("enm".into()), 1, Box::new(bool(true))),
        Token::Struct(Some("struct".into()), vec![
            Token::F32(Some("f32".into()), 3.14),
        ]),
        Token::Struct(Some("nested".into()), vec![
            Token::F32(Some("f32".into()), 2.7182),
        ]),
        Token::EmptyArr(Some("e_arr".into())),
        Token::U8Arr(Some("u8_arr".into()), vec![1, 2, 3].into()),
        Token::I32Arr(Some("i32_arr".into()), vec![1, 2, 3].into()),
        Token::I64Arr(Some("i64_arr".into()), vec![1, 2, 3].into()),
        Token::F32Arr(Some("f32_arr".into()), vec![1.0, 2.0, 3.0].into()),
        Token::F64Arr(Some("f64_arr".into()), vec![1.0, 2.0, 3.0].into()),
        Token::StrArr(Some("string_arr".into()), vec![
            "i".into(),
            "j".into(),
            "k".into(),
        ].into()),
        Token::TimestampMillis(Some("tsms".into()), 1),
        Token::TimestampMicros(Some("tsus".into()), 1),
        Token::MillisSinceBoot(Some("mssb".into()), 1),
        Token::MicrosSinceBoot(Some("ussb".into()), 1),
        Token::DurationMillis(Some("dnms".into()), 1),
        Token::DurationMicros(Some("dnus".into()), 1),
        Token::Vec2(Some("vec2".into()), [0.0, 0.0]),
        Token::Vec3(Some("vec3".into()), [0.0, 0.0, 0.0]),
        Token::Vec4(Some("vec4".into()), [0.0, 0.0, 0.0, 0.0]),
        Token::Quat(Some("quat".into()), [0.0, 0.0, 0.0, 0.0]),
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

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

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

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_u8() {
    let token = StructBuilder::new_root()
        .u8("u8", 42)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::U8(Some("u8".into()), 42),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_u16() {
    let token = StructBuilder::new_root()
        .u16("u16", 24930)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::U16(Some("u16".into()), 24930),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_u32() {
    let token = StructBuilder::new_root()
        .u32("u32", 1633837924)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::U32(Some("u32".into()), 1633837924),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_u64() {
    let token = StructBuilder::new_root()
        .u64("u64", 7017280452245743464)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::U64(Some("u64".into()), 7017280452245743464),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_i8() {
    let token = StructBuilder::new_root()
        .i8("i8", -15)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::I8(Some("i8".into()), -15),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_i16() {
    let token = StructBuilder::new_root()
        .i16("i16", -158)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::I16(Some("i16".into()), -158),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_i32() {
    let token = StructBuilder::new_root()
        .i32("i32", -10329244)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::I32(Some("i32".into()), -10329244),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_i64() {
    let token = StructBuilder::new_root()
        .i64("i64", -44363763471194264)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::I64(Some("i64".into()), -44363763471194264),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

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

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_f64() {
    let token = StructBuilder::new_root()
        .f64("f64", -4.035208983966375e+305)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::F64(Some("f64".into()), -4.035208983966375e+305),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

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

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_enum() {
    let token = StructBuilder::new_root()
        .enum_("enm", 1, bool(true))
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Enum(Some("enm".into()), 1, Box::new(bool(true))),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_struct() {
    let token = StructBuilder::new_root()
        .map(StructBuilder::new("struct").f32("f32", 3.14))
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Struct(Some("struct".into()), vec![
            Token::F32(Some("f32".into()), 3.14),
        ]),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_nested_struct() {
    let token = StructBuilder::new_root()
        .map(StructBuilder::new("nested").f32("f32", 2.7182))
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Struct(Some("nested".into()), vec![
            Token::F32(Some("f32".into()), 2.7182),
        ]),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

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

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_u8_arr() {
    let token = StructBuilder::new_root()
        .u8_arr("u8_arr", &[0x00, 0xFF, 0x33, 0x26])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::U8Arr(Some("u8_arr".into()), vec![0x00, 0xFF, 0x33, 0x26].into()),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_i32_arr() {
    let token = StructBuilder::new_root()
        .i32_arr("i32_arr", &[1, 2, 3])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::I32Arr(Some("i32_arr".into()), vec![1, 2, 3].into()),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_i64_arr() {
    let token = StructBuilder::new_root()
        .i64_arr("i64_arr", &[1, 2, 3])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::I64Arr(Some("i64_arr".into()), vec![1, 2, 3].into()),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_f32_arr() {
    let token = StructBuilder::new_root()
        .f32_arr("f32_arr", &[1.0, 2.0, 3.0])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::F32Arr(Some("f32_arr".into()), vec![1.0, 2.0, 3.0].into()),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_f64_arr() {
    let token = StructBuilder::new_root()
        .f64_arr("f64_arr", &[1.0, 2.0, 3.0])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::F64Arr(Some("f64_arr".into()), vec![1.0, 2.0, 3.0].into()),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_string_arr() {
    let token = StructBuilder::new_root()
        .string_arr("string_arr", &["a", "b"])
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::StrArr(Some("string_arr".into()), vec!["a".into(), "b".into()].into()),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

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

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_timestamp_us() {
    let token = StructBuilder::new_root()
        .timestamp_us("tsus", 7017280452245743464)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::TimestampMicros(Some("tsus".into()), 7017280452245743464),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_ms_since_boot() {
    let token = StructBuilder::new_root()
        .ms_since_boot("mssb", 7017280452245743464)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::MillisSinceBoot(Some("mssb".into()), 7017280452245743464),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_us_since_boot() {
    let token = StructBuilder::new_root()
        .us_since_boot("ussb", 7017280452245743464)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::MicrosSinceBoot(Some("ussb".into()), 7017280452245743464),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_duration_ms() {
    let token = StructBuilder::new_root()
        .duration_ms("dnms", -44363763471194264)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::DurationMillis(Some("dnms".into()), -44363763471194264),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_duration_us() {
    let token = StructBuilder::new_root()
        .duration_us("dnus", -44363763471194264)
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::DurationMicros(Some("dnus".into()), -44363763471194264),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_vec2() {
    let token = StructBuilder::new_root()
        .vec2("vec2", Vec2::new(0.0, 1.0).as_arr())
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Vec2(Some("vec2".into()), [0.0, 1.0]),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_vec3() {
    let token = StructBuilder::new_root()
        .vec3("vec3", Vec3::new(0.0, 1.0, 2.0).as_arr())
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Vec3(Some("vec3".into()), [0.0, 1.0, 2.0]),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_vec4() {
    let token = StructBuilder::new_root()
        .vec4("vec4", Vec4::new(0.0, 1.0, 2.0, 3.0).as_arr())
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Vec4(Some("vec4".into()), [0.0, 1.0, 2.0, 3.0]),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}

#[test]
fn serialize_deserialize_quaternion() {
    let token = StructBuilder::new_root()
        .quaternion("quat", Quat::new(0.0, 1.0, 2.0, 3.0).as_arr())
        .build();
    
    let expected_token = Token::Struct(None, vec![
        Token::Quat(Some("quat".into()), [0.0, 1.0, 2.0, 3.0]),
    ]);

    let bytes = as_bytes(&token).unwrap();
    let result = from_bytes(&bytes).unwrap();

    assert_eq!(result, expected_token)
}