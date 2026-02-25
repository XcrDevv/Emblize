// use emblize::{Builder, as_bytes, from_bytes};

// use std::fs;

use emblize::{deserialize, ser::serialize_to_vec};
// use emblize::{StructBuilder, deserialize, ser::serialize_to_vec};
// use emblize::{Builder, as_bytes, deserialize, from_bytes, ser::serialize_to_alloc_vec};
use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
// struct Led<'a> {
//     pub c: &'a str,
//     pub v: bool
// }

#[derive(Deserialize, Serialize, Debug)]
enum Command {
    #[serde(rename = "LDA")]
    LedA { v: bool },

    #[serde(rename = "LDB")]
    LedB { v: bool },
}

fn main() {
    // let token = StructBuilder::new_root()
    //     .string("c", "LED")
    //     .bool("v", true)
    //     .build();
    
    // let bytes_b = as_bytes(&token).unwrap_or_else(|e| panic!("{}", e));
    let bytes_s = serialize_to_vec::<Command, 128>(&Command::LedA { v: true }).unwrap_or_else(|e| panic!("{}", e));
    // fs::write("/output/data.dat", &bytes_s).unwrap();

    // let b = from_bytes(&bytes_b).unwrap_or_else(|e| panic!("{}", e));
    // println!("{:#?}", b);

    let s: Command = deserialize(&bytes_s).unwrap_or_else(|e| panic!("{}", e));
    println!("{:?}", s)
}