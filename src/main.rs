
use emblize::{as_bytes, from_bytes, Builder, StructBuilder};

fn main() {
    let data = Builder::new()
        .string("c", "CNT".into())
        .build();
    // let data = Builder::new()
    //     .f32("x", 1.0)
    //     .f32("y", 2.0)
    //     .f32("z", 3.0)
    //     .build();

    let content_bytes = as_bytes(&data).unwrap();
    std::fs::write("./output/data.dat", content_bytes).unwrap();
    let file_bytes = std::fs::read("./output/data.dat").unwrap();

    let data_read = from_bytes(&file_bytes).unwrap();

    println!("{:#?}", data_read)
}