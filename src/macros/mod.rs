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
/// The type must have fields with the exact names provided in the macro invocation.
/// All fields are assumed to be `f32` types.
///
/// # Examples
///
/// ```rust,ignore
/// use serde::Serialize;
///
/// struct Vec3 {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// impl_serialize_vec!(Vec3, x, y, z);
///
/// let v = Vec3 { x: 1.0, y: 2.0, z: 3.0 };
/// let bytes = serialize(&v).unwrap();
/// ```
///
/// ```rust,ignore
/// use serde::Serialize;
///
/// struct Quat {
///     x: f32,
///     y: f32,
///     z: f32,
///     w: f32,
/// }
///
/// impl_serialize_vec!(Quat, x, y, z, w);
/// ```
///
/// # Wire Format
///
/// The serialized data uses the binary protocol's `Vec2F32`, `Vec3F32`, `Vec4F32`, or 
/// `QuatF32` token types depending on the number of fields, ensuring efficient 
/// transmission for embedded systems.
#[macro_export]
macro_rules! impl_serialize_vec {
    (
        $VecN:ident,
        $( $field:ident ),+
    ) => {
        impl serde::Serialize for $VecN
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
/// a newtype struct containing a sequence of floats. The deserialized format matches 
/// the custom binary protocol used in embedded/robotics systems.
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
/// - Have all fields as `f32` types
/// - Have fields in the order specified (order matters for deserialization)
///
/// # Examples
///
/// ```rust,ignore
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq)]
/// struct Vec3 {
///     x: f32,
///     y: f32,
///     z: f32,
/// }
///
/// impl_deserialize_vec!(Vec3, x, y, z);
///
/// let bytes = /* binary data */;
/// let v: Vec3 = deserialize(&bytes).unwrap();
/// assert_eq!(v, Vec3 { x: 1.0, y: 2.0, z: 3.0 });
/// ```
///
/// ```rust,ignore
/// use serde::Deserialize;
///
/// #[derive(Debug, PartialEq)]
/// struct Quat {
///     x: f32,
///     y: f32,
///     z: f32,
///     w: f32,
/// }
///
/// impl_deserialize_vec!(Quat, x, y, z, w);
/// ```
///
/// # Error Handling
///
/// Returns a deserialization error if:
/// - The sequence has fewer elements than expected fields
/// - The data type doesn't match the expected format
/// - The binary format is corrupted
///
/// # Wire Format
///
/// Expects data serialized with the binary protocol's `Vec2F32`, `Vec3F32`, `Vec4F32`, 
/// or `QuatF32` token types depending on the number of fields.
#[macro_export]
macro_rules! impl_deserialize_vec {
    (
        $VecN:ident,
        $( $field:ident ),+
    ) => {
        impl<'de> serde::Deserialize<'de> for $VecN {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct V;

                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $VecN;

                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        write!(
                            formatter,
                            "{} as [{}]",
                            stringify!($VecN),
                            stringify!($( $field f32 ),+)
                        )
                    }

                    fn visit_newtype_struct<D>(self, deserializer: D) -> core::result::Result<Self::Value, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        deserializer.deserialize_seq(self)
                    }

                    fn visit_seq<A>(self, mut seq: A) -> core::result::Result<Self::Value, A::Error>
                    where
                        A: serde::de::SeqAccess<'de>,
                    {
                        let mut iter = (0usize..).into_iter();

                        $(
                            let i = iter.next().unwrap();
                            let $field: f32 = seq
                                .next_element()?
                                .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;
                        )+
                        Ok($VecN { $( $field ),+ })
                    }
                }

                deserializer.deserialize_newtype_struct(stringify!($VecN), V)
            }
        }
    };
}