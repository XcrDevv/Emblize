/// Implements `serde::Serialize` for vector-like types with named fields.
///
/// This macro generates a `Serialize` implementation that serializes the type as a 
/// newtype struct containing a tuple of its fields. The serialized format is optimized 
/// for the custom binary protocol used in embedded/robotics systems.
///
/// # Parameters
///
/// - `$VecN` - The name of the type to implement `Serialize` for
/// - `$field` - One or more field names (e.g., `x, y, z`)
///
/// # Requirements
///
/// The type must:
/// - Have fields with the exact names provided in the macro invocation
/// - Have all fields as `T` where `T: VectorNumber`
///
/// # Examples
///
/// ```rust,ignore
/// use serde::Serialize;
///
/// struct Vec3<T> {
///     x: T,
///     y: T,
///     z: T,
/// }
///
/// impl_serialize_vec!(Vec3, x, y, z);
///
/// let v = Vec3 { x: 1.0f32, y: 2.0f32, z: 3.0f32 };
/// let bytes = serialize(&v).unwrap();
/// ```
///
/// # Wire Format
///
/// The serialized data uses the binary protocol's `Vec2`, `Vec3` or `Vec4`
/// token types depending on the number of fields, supporting any numeric 
/// type `T` that implements `VectorNumber` (e.g., `f32`, `f64`, `i32`).
#[macro_export]
macro_rules! impl_serialize_vec {
    (
        $VecN:ident,
        $( $field:ident ),+
    ) => {
        impl<T> serde::Serialize for $VecN<T>
        where 
            T: crate::core::types::VectorNumber
        {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_newtype_struct(
                    stringify!($VecN),
                    &(
                        $( self.$field ),+
                    )
                )
            }
        }
    };
}

/// Implements `serde::Deserialize` for vector-like types with named fields.
///
/// This macro generates a `Deserialize` implementation that deserializes the type from 
/// a newtype struct containing a sequence of numeric values. The deserialized format 
/// matches the custom binary protocol used in embedded/robotics systems.
///
/// # Parameters
///
/// - `$VecN` - The name of the type to implement `Deserialize` for
/// - `$field` - One or more field names (e.g., `x, y, z`)
///
/// # Requirements
///
/// The type must:
/// - Have fields with the exact names provided in the macro invocation
/// - Have all fields as `T` where `T: VectorNumber + serde::Deserialize`
/// - Have fields in the order specified (order matters for deserialization)
///
/// # Example
///
/// ```rust,ignore
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq)]
/// struct Vec3<T> {
///     x: T,
///     y: T,
///     z: T,
/// }
///
/// impl_deserialize_vec!(Vec3, x, y, z);
///
/// let bytes = vec![0x0A, 0x40, 0xC9, 0x0F, 0xDB];
/// let v: Vec3<f32> = deserialize(&bytes).unwrap();
/// ```
///
/// # Error Handling
///
/// Returns a deserialization error if:
/// - The sequence has fewer elements than expected fields
/// - The element type doesn't match `T`
/// - The binary format is corrupted
///
/// # Wire Format
///
/// Expects data serialized with the binary protocol's `Vec2`, `Vec3` or `Vec4`
/// token types depending on the number of fields, supporting any
/// numeric type `T` that implements `VectorNumber`.
#[macro_export]
macro_rules! impl_deserialize_vec {
    (
        $VecN:ident,
        $( $field:ident ),+
    ) => {
        impl<'de, T> serde::Deserialize<'de> for $VecN<T>
        where
            T: crate::core::types::VectorNumber + 'de
        {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V<T>(core::marker::PhantomData<T>);

                impl<'de, T> serde::de::Visitor<'de> for V<T>
                where
                    T: crate::core::types::VectorNumber + 'de
                {
                    type Value = $VecN<T>;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        write!(
                            formatter,
                            "{} as [{}]",
                            stringify!($VecN),
                            stringify!($( $field T ),+)
                        )
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> core::result::Result<Self::Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        deserializer.deserialize_tuple(LEN, self)
                    }

                    fn visit_seq<A>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let mut iter = (0usize..).into_iter();

                        $(
                            let i = iter.next().unwrap();
                            let $field: T = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                        )+

                        Ok($VecN::new($( $field ),+))
                    }
                }

                const LEN: usize = impl_deserialize_vec!(@count $( $field ),+);

                deserializer.deserialize_newtype_struct(stringify!($VecN), V::<T>(core::marker::PhantomData))
            }
        }
    };

    (@count $($field:ident),+) => {
        <[()]>::len(&[$(impl_deserialize_vec!(@replace $field)),+])
    };

    (@replace $field:ident) => { () };
}