pub mod factory;
pub mod builder;
mod decoder;
mod encoder;

pub use builder::StructBuilder;
pub use encoder::encode;
pub use decoder::decode;