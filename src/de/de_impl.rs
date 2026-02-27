use crate::{
    de::deserializer::{DeState, Deserializer},
    error::{Error, Result},
    impl_deserialize_vec,
    core::read_write::Reader,
    core::token::TokenTag,
    core::types::*,
};
use serde::de::{self, EnumAccess, MapAccess, SeqAccess, VariantAccess};

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input: Reader::new(input),
            state: DeState::ReadingField,
        }
    }
}

pub fn deserialize<'de, T>(s: &'de [u8]) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    Ok(t)
}

impl<'de> Deserializer<'de> {
    fn read_bool(&mut self) -> Result<bool> {
        match self.input.read_byte()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::ExpectedType("Bool")),
        }
    }

    fn read_str(&mut self) -> Result<&'de str> {
        let len = self.input.read_number::<u16>()? as usize;
        let bytes = self.input.take_bytes(len)?;
        core::str::from_utf8(bytes).map_err(|_| Error::NoUTF8)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::Bool.matches(self.input.read_byte()?)?;
        }

        visitor.visit_bool(self.read_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::I8.matches(self.input.read_byte()?)?;
        }

        visitor.visit_i8(self.input.read_number()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::I16.matches(self.input.read_byte()?)?;
        }

        visitor.visit_i16(self.input.read_number()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::I32.matches(self.input.read_byte()?)?;
        }

        visitor.visit_i32(self.input.read_number()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::I64.matches(self.input.read_byte()?)?;
        }

        visitor.visit_i64(self.input.read_number()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::U8.matches(self.input.read_byte()?)?;
        }

        visitor.visit_u8(self.input.read_number()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::U16.matches(self.input.read_byte()?)?;
        }

        visitor.visit_u16(self.input.read_number()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::U32.matches(self.input.read_byte()?)?;
        }

        visitor.visit_u32(self.input.read_number()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::U64.matches(self.input.read_byte()?)?;
        }

        visitor.visit_u64(self.input.read_number()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::F32.matches(self.input.read_byte()?)?;
        }

        visitor.visit_f32(self.input.read_number()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::F64.matches(self.input.read_byte()?)?;
        }

        visitor.visit_f64(self.input.read_number()?)
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeUnsupported("char"))
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::Str.matches(self.input.read_byte()?)?;
        }
        let s = self.read_str()?;
        visitor.visit_borrowed_str(s)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::Str.matches(self.input.read_byte()?)?;
        }
        self.deserialize_str(visitor)
    }

    #[cfg(feature = "alloc")]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadingField {
            TokenTag::Str.matches(self.input.read_byte()?)?;
        }
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeUnsupported("&[u8]"))
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        // TODO
        // TODO
        // TODO
        unimplemented!()
    }

    fn deserialize_option<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeUnsupported("Option<T>"))
    }

    fn deserialize_unit<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeUnsupported("unit ()"))
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        Err(Error::DeUnsupported("unit struct"))
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        match name {
            "Vec2" => {
                TokenTag::Vec2.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingFixedSeq(2);
            }
            "Vec3" => {
                TokenTag::Vec3.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingFixedSeq(3);
            }
            "Vec4" => {
                TokenTag::Vec4.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingFixedSeq(4);
            }
            "Quat" => {
                TokenTag::Quat.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingFixedSeq(4);
            }
            "TimestampMillis" => {
                TokenTag::TimestampMillis.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingTime;
            }
            "TimestampMicros" => {
                TokenTag::TimestampMicros.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingTime;
            }
            "MillisSinceBoot" => {
                TokenTag::MillisSinceBoot.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingTime;
            }
            "MicrosSinceBoot" => {
                TokenTag::MicrosSinceBoot.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingTime;
            }
            "DurationMillis" => {
                TokenTag::DurationMillis.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingTime;
            }
            "DurationMicros" => {
                TokenTag::DurationMicros.matches(self.input.read_byte()?)?;
                self.state = DeState::ReadingTime;
            }
            _ => unreachable!("{}", name),
        }

        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = match self.state {
            DeState::ReadingFixedSeq(len) => len,
            _ => {
                self.state = DeState::ReadingSeq;
                if self.input.read_byte()? == TokenTag::EmptyArr as u8 {
                    0
                } else {
                    self.input.read_number::<u16>()? as usize
                }
            }
        };

        visitor.visit_seq(SizedCollection {
            de: self,
            remaining: len,
        })
    }

    fn deserialize_tuple<V>(self, tup_len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.input.read_byte()?;
        let len = self.input.read_number::<u16>()? as usize;

        if len != tup_len {
            return Err(Error::MissmatchLength { expected: tup_len, found: len });
        }

        self.state = DeState::ReadingSeq;

        visitor.visit_seq(SizedCollection {
            de: self,
            remaining: len,
        })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_tuple(len, visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let field_count = self.input.read_byte()?;

        visitor.visit_map(SizedCollection {
            de: self,
            remaining: field_count as usize,
        })
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let tag = self.input.read_byte()?;

        TokenTag::Struct.matches(tag)?;

        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        TokenTag::Enum.matches(self.input.read_byte()?)?;
        visitor.visit_enum(Enum { de: self })
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_str(self.read_str()?)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct SizedCollection<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    remaining: usize,
}

impl<'de, 'a> SeqAccess<'de> for SizedCollection<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: de::DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            self.de.state = DeState::ReadingField;
            return Ok(None);
        }

        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
}

impl<'de, 'a> MapAccess<'de> for SizedCollection<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: de::DeserializeSeed<'de>,
    {
        if self.remaining == 0 {
            self.de.state = DeState::ReadingField;
            return Ok(None);
        }

        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn size_hint(&self) -> Option<usize> {
        Some(self.remaining)
    }
}

struct Enum<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'de, 'a> EnumAccess<'de> for Enum<'a, 'de> {
    type Error = Error;
    type Variant = Enum<'a, 'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: de::DeserializeSeed<'de>,
    {
        let index = self.de.input.read_byte()?;
        let value = seed.deserialize(
            serde::de::value::U8Deserializer::<Error>::new(index)
        )?;
        Ok((value, self))
    }
}

impl<'de, 'a> VariantAccess<'de> for Enum<'a, 'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: de::DeserializeSeed<'de>,
    {
        seed.deserialize(&mut *self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_seq(SizedCollection {
            de: self.de,
            remaining: len,
        })
    }

    fn struct_variant<V>(self, fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        TokenTag::Struct.matches(self.de.input.read_byte()?)?;
        let len = self.de.input.read_number::<u8>()? as usize;
        if len != fields.len() {
            return Err(Error::MissmatchLength { expected: fields.len(), found: len });
        }

        visitor.visit_map(SizedCollection {
            de: self.de,
            remaining: fields.len(),
        })
    }
}

macro_rules! impl_deserialize_time {
    (
        $Time:ident,
        $ty:ty,
        $visit:ident,
        $de:ident
    ) => {
        impl<'de> serde::Deserialize<'de> for $Time {
            fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
            where
            D: de::Deserializer<'de>,
            {
                struct V;
                
                impl<'de> serde::de::Visitor<'de> for V {
                    type Value = $Time;
                    
                    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                        write!(formatter, "{}({})", stringify!(&Time), stringify!($ty),)
                    }
                    
                    fn visit_newtype_struct<D>(
                        self,
                        deserializer: D,
                    ) -> core::result::Result<Self::Value, D::Error>
                    where
                    D: serde::Deserializer<'de>,
                    {
                        deserializer.$de(self)
                    }
                    
                    fn $visit<E>(self, v: $ty) -> core::result::Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        Ok($Time(v))
                    }
                }
                deserializer.deserialize_newtype_struct(stringify!($Time), V)
            }
        }
    };
}

impl_deserialize_time!(TimestampMillis, u64, visit_u64, deserialize_u64);
impl_deserialize_time!(TimestampMicros, u64, visit_u64, deserialize_u64);
impl_deserialize_time!(MillisSinceBoot, u64, visit_u64, deserialize_u64);
impl_deserialize_time!(MicrosSinceBoot, u64, visit_u64, deserialize_u64);
impl_deserialize_time!(DurationMillis, i64, visit_i64, deserialize_i64);
impl_deserialize_time!(DurationMicros, i64, visit_i64, deserialize_i64);

impl_deserialize_vec!(Vec2, x, y);
impl_deserialize_vec!(Vec3, x, y, z);
impl_deserialize_vec!(Vec4, x, y, z, w);
impl_deserialize_vec!(Quat, x, y, z, w);
