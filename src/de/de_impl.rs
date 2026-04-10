use crate::{
    core::{reader::Reader, token::TokenTag, types::*}, de::deserializer::{DeState, Deserializer}, error::{Error, Result}, impl_deserialize_vec
};
use num_enum::TryFromPrimitive;
use serde::de::{self, EnumAccess, MapAccess, SeqAccess, VariantAccess};

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer {
            input: Reader::new(input),
            state: DeState::ReadTypedValue,
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
        let len = self.read_variant_usize()?;
        let bytes = self.input.take_bytes(len)?;
        core::str::from_utf8(bytes).map_err(|_| Error::NoUTF8)
    }

    fn expected_tag(&mut self, token_tag: TokenTag) -> Result<()> {
        match self.state {
            DeState::ReadTypedValue => {
                token_tag.matches(self.input.read_byte()?)?;
            }
            DeState::ReadSeq(ref mut arr_type) => {
                if let Some(arr_type) = arr_type.take() {
                    token_tag.matches(arr_type)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    fn prepare_seq(&mut self, expected_len: Option<usize>) -> Result<usize> {
        let len = match self.state {
            DeState::ReadSeq(_) => {
                let len = self.read_variant_usize()?;
                let arr_type = self.input.read_byte()?;
                self.state = DeState::ReadSeq(Some(arr_type));
                len
            }
            _ => {
                let tag = self.input.read_byte()?;
                if tag == TokenTag::EmptyArr as u8 {
                    0
                } else {
                    let len = self.read_variant_usize()?;
                    let arr_type = self.input.read_byte()?;
                    self.state = DeState::ReadSeq(Some(arr_type));
                    len
                }
            }
        };

        if let Some(expected) = expected_len {
            if len != expected {
                return Err(Error::MissmatchLength { expected, got: len });
            }
        }

        Ok(len)
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::Bool)?;
        visitor.visit_bool(self.read_bool()?)
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::I8)?;
        let v = self.input.read_number()?;
        visitor.visit_i8(v)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::I16)?;
        let v = self.input.read_number()?;
        visitor.visit_i16(v)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::I32)?;
        let v = self.input.read_number()?;
        visitor.visit_i32(v)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::I64)?;
        let v = self.input.read_number()?;
        visitor.visit_i64(v)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::U8)?;
        let v = self.input.read_number()?;
        visitor.visit_u8(v)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::U16)?;   
        let v = self.input.read_number()?;
        visitor.visit_u16(v)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::U32)?;
        let v = self.input.read_number()?;
        visitor.visit_u32(v)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::U64)?;
        let v = self.input.read_number()?;
        visitor.visit_u64(v)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::F32)?;
        let v = self.input.read_number()?;
        visitor.visit_f32(v)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::F64)?;
        let v = self.input.read_number()?;
        visitor.visit_f64(v)
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
        self.expected_tag(TokenTag::Str)?;
        let s = self.read_str()?;
        visitor.visit_borrowed_str(s)
    }

    #[cfg(not(feature = "alloc"))]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    #[cfg(feature = "alloc")]
    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::Str)?;
        visitor.visit_string(self.read_string()?)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.expected_tag(TokenTag::Bytes)?;
        let len = self.read_variant_usize()?;
        let bytes = self.input.take_bytes(len)?;
        visitor.visit_borrowed_bytes(bytes)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let token = self.input.read_byte()?;
        
        match self.state {
            DeState::ReadTypedValue => {
                if TokenTag::Some as u8 != token && TokenTag::None as u8 != token {
                    return Err(Error::ExpectedType("Some or None"))
                }
            }
            DeState::ReadSeq(ref mut arr_type) => {
                if let Some(arr_type) = arr_type.take() {
                    if TokenTag::Some as u8 != arr_type && TokenTag::None as u8 != arr_type {
                        return Err(Error::ExpectedType("Some or None"))
                    }
                }
            }
            _ => {}
        }

        match TokenTag::try_from_primitive(token).map_err(|_| Error::InvalidToken)? {
            TokenTag::Some => {
                visitor.visit_some(self)
            },
            TokenTag::None => {
                visitor.visit_none()
            },
            _ => Err(Error::ExpectedType("Some or None"))
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        visitor.visit_unit()
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
                self.expected_tag(TokenTag::Vec2)?;
                self.state = DeState::ReadVec
            }
            "Vec3" => {
                self.expected_tag(TokenTag::Vec3)?;
                self.state = DeState::ReadVec
            }
            "Vec4" => {
                self.expected_tag(TokenTag::Vec4)?;
                self.state = DeState::ReadVec
            }
            "Quat" => {
                self.expected_tag(TokenTag::Quat)?;
                self.state = DeState::ReadVec
            }
            "TimestampMillis" => {
                self.expected_tag(TokenTag::TimestampMillis)?;
                self.state = DeState::ReadUntypedValue;
            }
            "TimestampMicros" => {
                self.expected_tag(TokenTag::TimestampMicros)?;
                self.state = DeState::ReadUntypedValue;
            }
            "MillisSinceBoot" => {
                self.expected_tag(TokenTag::MillisSinceBoot)?;
                self.state = DeState::ReadUntypedValue;
            }
            "MicrosSinceBoot" => {
                self.expected_tag(TokenTag::MicrosSinceBoot)?;
                self.state = DeState::ReadUntypedValue;
            }
            "DurationMillis" => {
                self.expected_tag(TokenTag::DurationMillis)?;
                self.state = DeState::ReadUntypedValue;
            }
            "DurationMicros" => {
                self.expected_tag(TokenTag::DurationMicros)?;
                self.state = DeState::ReadUntypedValue;
            }
            _ => unreachable!(),
        }

        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        let len = self.prepare_seq(None)?;

        visitor.visit_seq(SizedCollection {
            de: self,
            remaining: len,
        })
    }

    fn deserialize_tuple<V>(self, tup_len: usize, visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        if self.state == DeState::ReadVec {
            self.input.read_byte()?;
            return visitor.visit_seq(SizedCollection {
                de: self,
                remaining: tup_len,
            })
        }
        
        let len = self.prepare_seq(Some(tup_len))?;

        if len != tup_len {
            return Err(Error::MissmatchLength { expected: tup_len, got: len });
        }

        visitor.visit_seq(SizedCollection {
            de: self,
            remaining: tup_len,
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

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: de::Visitor<'de>,
    {
        unimplemented!("Maps deserialization hasn't yet been implemented"); // ! CHECK
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
        self.expected_tag(TokenTag::Struct)?; 
        let field_count = self.read_variant_usize()?;
        visitor.visit_map(SizedCollection {
            de: self,
            remaining: field_count,
        })
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
        self.expected_tag(TokenTag::Enum)?;
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
            self.de.state = DeState::ReadTypedValue;
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
            return Ok(None);
        }

        self.remaining -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }


    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: de::DeserializeSeed<'de>,
    {
        let prev_state = self.de.state;
        self.de.state = DeState::ReadTypedValue;
        let result = seed.deserialize(&mut *self.de);
        self.de.state = prev_state;
        result
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
            serde::de::value::U8Deserializer::<Error>::new(index & 0x7F)
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
        let obtained_len = self.de.prepare_seq(Some(len))?;

        if len != len {
            return Err(Error::MissmatchLength { expected: len, got: obtained_len });
        }

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
        let len = self.de.read_variant_usize()?;
        if len != fields.len() {
            return Err(Error::MissmatchLength { expected: fields.len(), got: len });
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

impl<'de> serde::Deserialize<'de> for Quat
{
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct V;

        impl<'de> serde::de::Visitor<'de> for V
        {
            type Value = Quat;

            fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(
                    formatter,
                    "Quat as [x f32, y f32, z f32, w f32]"
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

                let i = iter.next().unwrap();
                let x: f32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;

                let i = iter.next().unwrap();
                let y: f32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;

                let i = iter.next().unwrap();
                let z: f32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;

                let i = iter.next().unwrap();
                let w: f32 = seq
                    .next_element()?
                    .ok_or_else(|| serde::de::Error::invalid_length(i, &self))?;

                Ok(Quat::new(x, y, z, w))
            }
        }

        const LEN: usize = 4;

        deserializer.deserialize_newtype_struct("Quat", V)
    }
}