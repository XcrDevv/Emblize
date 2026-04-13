#![cfg(feature = "alloc")]

use emblize::{core::token::TokenTag, from_bytes, to_allocvec};
use serde::{Deserialize, Serialize};

const STRUCT_PREFIX: &[u8] = &[TokenTag::Struct as u8, 0x01, 0x00, 0x01, 0x76];

fn make_fake_value(suffix: &[u8]) -> Vec<u8> {
    let mut v = Vec::from(STRUCT_PREFIX);
    v.extend_from_slice(suffix);
    v
}

#[test]
#[should_panic]
fn unsupported_serialize_char() {
    #[derive(Serialize)]
    struct Root {
        v: char
    }

    let value = Root { v: 'a' };
    let _ = to_allocvec(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_deserialize_char() {
    #[derive(Deserialize)]
    struct Root {
        #[allow(dead_code)]
        v: char
    }

    let value = make_fake_value(&[0x00, 0x00]);
    let _ = from_bytes::<Root>(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_serialize_tuple_t() {
    #[derive(Serialize)]
    struct Root {
        v: (i32, u32)
    }

    let value = Root { v: (1, 2) };
    let _ = to_allocvec(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_deserialize_tuple_t() {
    let value = [TokenTag::Array as u8, TokenTag::I32 as u8, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x02];
    let _ = from_bytes::<(i32, u32)>(&value).unwrap();
}