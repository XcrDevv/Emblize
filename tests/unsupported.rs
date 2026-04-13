#![cfg(feature = "alloc")]

use emblize::{core::token::TokenTag, from_bytes, to_allocvec};
use serde::Serialize;

#[test]
#[should_panic]
fn unsupported_serialize_char() {
    let _ = to_allocvec(&'f').unwrap();
}

#[test]
#[should_panic]
fn unsupported_deserialize_char() {
    let _ = from_bytes::<char>(&[0x00, 0x00]).unwrap();
}

#[test]
#[should_panic]
fn unsupported_serialize_heterogeneous_tuple() {
    #[derive(Serialize)]
    struct Root {
        v: (i32, u32)
    }

    let value = Root { v: (1, 2) };
    let _ = to_allocvec(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_deserialize_heterogeneous_tuple() {
    let value = [TokenTag::Array as u8, TokenTag::I32 as u8, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let _ = from_bytes::<(i32, u32)>(&value).unwrap();
}