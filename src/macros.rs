/// A trait for number types that can be serialized to and deserialized from
/// big-endian byte representation
pub trait BytesNum {
    /// Converts a big-endiand byte slice into a value
    /// # Panics 
    /// Panics if the input slice has an incorrect length
    fn from_be_bytes(bytes: &[u8]) -> Self;

    /// Converts a value into its big-endian byte representation
    fn to_be_bytes(&self) -> Vec<u8>;
}

macro_rules! impl_number_be_bytes {
    ($t:ty) => {
        impl BytesNum for $t {
            fn from_be_bytes(bytes: &[u8]) -> Self {
                let arr: [u8; std::mem::size_of::<$t>()] = bytes.try_into().expect("Invalid size");
                <$t>::from_be_bytes(arr)
            }
            fn to_be_bytes(&self) -> Vec<u8> {
                <$t>::to_be_bytes(*self).to_vec()
            }
        }
    };
}

// Implements the `BytesNum` trait to all number types used in `Token`
impl_number_be_bytes!(u8);
impl_number_be_bytes!(u16);
impl_number_be_bytes!(u32);
impl_number_be_bytes!(u64);
impl_number_be_bytes!(i32);
impl_number_be_bytes!(i64);
impl_number_be_bytes!(f32);
impl_number_be_bytes!(f64);