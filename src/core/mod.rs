pub mod token;
pub mod math;
pub mod time;
pub mod utils;
pub mod frame;
pub mod read_write;

// #[cfg(feature = "alloc")]
// mod imp {
//     pub type Vec<T> = alloc::vec::Vec<T>;
// }

// #[cfg(not(feature = "alloc"))]
// mod imp {
//     pub type Vec<T> = heapless::Vec<T, 16>;
// }

// pub use imp::Vec;