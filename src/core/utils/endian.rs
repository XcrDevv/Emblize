/// A trait for number types that can be serialized to and deserialized from
/// big-endian byte representation
pub trait BytesNum: Sized {
    type Bytes: Copy + AsRef<[u8]> + for<'a> TryFrom<&'a [u8]>;
    fn from_be_bytes(bytes: Self::Bytes) -> Self;
    fn to_be_bytes(&self) -> Self::Bytes;
}

macro_rules! impl_number_be_bytes {
    ($t:ty) => {
        impl BytesNum for $t {
            type Bytes = [u8; core::mem::size_of::<$t>()];

            fn from_be_bytes(bytes: Self::Bytes) -> Self {
                <$t>::from_be_bytes(bytes)
            }

            fn to_be_bytes(&self) -> Self::Bytes {
                <$t>::to_be_bytes(*self)
            }
        }
    };
}

impl_number_be_bytes!(u8);
impl_number_be_bytes!(u16);
impl_number_be_bytes!(u32);
impl_number_be_bytes!(u64);
impl_number_be_bytes!(i8);
impl_number_be_bytes!(i16);
impl_number_be_bytes!(i32);
impl_number_be_bytes!(i64);
impl_number_be_bytes!(f32);
impl_number_be_bytes!(f64);