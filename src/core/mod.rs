mod math;
mod time;
pub mod token;
pub mod utils;
pub mod frame;
pub mod reader;


pub mod types {
    pub use super::time::*;
    pub use super::math::*;
}

// #[cfg(feature = "alloc")]
// mod imp {
//     pub type Vec<T> = alloc::vec::Vec<T>;
// }

// #[cfg(not(feature = "alloc"))]
// mod imp {
//     pub type Vec<T> = heapless::Vec<T, 16>;
// }

// pub use imp::Vec;