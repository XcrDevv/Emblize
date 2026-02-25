#![cfg(feature = "alloc")]

use emblize::{core::token::TokenTag, deserialize, ser::serialize_to_alloc_vec};
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
    let _ = serialize_to_alloc_vec(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_serialize_none() {
    #[derive(Serialize)]
    struct Root {
        v: Option<u8>
    }

    let value = Root { v: None };
    let _ = serialize_to_alloc_vec(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_serialize_some() {
    #[derive(Serialize)]
    struct Root {
        v: Option<u8>
    }

    let value = Root { v: Some(0) };
    let _ = serialize_to_alloc_vec(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_serialize_unit() {
    let _ = serialize_to_alloc_vec(&()).unwrap();
}

#[test]
#[should_panic]
fn unsupported_serialize_unit_struct() {
    #[derive(Serialize)]
    struct S;

    #[derive(Serialize)]
    struct Root {
        v: S
    }

    let value = Root { v: S };
    let _ = serialize_to_alloc_vec(&value).unwrap();
}


#[test]
#[should_panic]
fn unsupported_serialize_newtype_variant() {
    #[derive(Serialize)]
    enum E {
        VariantA(u8)
    }

    #[derive(Serialize)]
    struct Root {
        v: E
    }

    let value = Root { v: E::VariantA(0) };
    let _ = serialize_to_alloc_vec(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_serialize_tuple_variant() {
    #[derive(Serialize)]
    enum E {
        VariantA(u8, u8)
    }

    #[derive(Serialize)]
    struct Root {
        v: E
    }

    let value = Root { v: E::VariantA(0, 0) };
    let _ = serialize_to_alloc_vec(&value).unwrap();
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
    let _ = deserialize::<Root>(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_deserialize_some() {
    #[derive(Deserialize)]
    struct Root {
        #[allow(dead_code)]
        v: Option<u8>
    }

    let value = make_fake_value(&[0x00, 0x00]);
    let _ = deserialize::<Root>(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_deserialize_bytes() {
    #[derive(Deserialize)]
    struct Root<'a> {
        #[allow(dead_code)]
        v: &'a [u8]
    }

    let value = make_fake_value(&[0x00, 0x00]);
    let _ = deserialize::<Root>(&value).unwrap();
}

#[test]
#[should_panic]
fn unsupported_deserialize_unit() {
    let value = make_fake_value(&[0x00, 0x00]);
    let _ = deserialize::<()>(&value).unwrap();
}


#[test]
#[should_panic]
fn unsupported_deserialize_unit_struct() {
    #[derive(Deserialize)]
    struct S;

    #[derive(Deserialize)]
    struct Root {
        #[allow(dead_code)]
        v: S
    }

    let value = make_fake_value(&[0x00, 0x00]);
    let _ = deserialize::<Root>(&value).unwrap();
}