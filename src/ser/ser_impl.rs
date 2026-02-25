use crate::{
    core::{math::*, time::*, token::TokenTag}, 
    error::{Error, Result}, 
    impl_serialize_vec, 
    ser::serializer::{SerState, Serializer, SerializerBuf}
};
use serde::{
    Serialize,
    ser::{self},
};

pub fn serialize<T, B: SerializerBuf>(value: T) -> Result<B>
where
    T: Serialize,
{
    let mut serializer: Serializer<B> = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.buf)
}

pub struct SeqSerializer<'a, B: SerializerBuf> {
    ser: &'a mut Serializer<B>,
}

impl<'a, B: SerializerBuf> ser::Serializer for &'a mut Serializer<B> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, B>;
    type SerializeTuple = SeqSerializer<'a, B>;
    type SerializeTupleStruct = SeqSerializer<'a, B>;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn collect_str<T>(self, _value: &T) -> core::result::Result<Self::Ok, Self::Error>
        where
            T: ?Sized + core::fmt::Display, {
        todo!()
    }

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        if !(self.state == SerState::WritingField) {
            return Err(Error::DTypeNotSupported(self.state.as_str()));
        }

        self.buf.push_byte(TokenTag::Bool as u8)?;
        self.buf.push_byte(if v { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        if !(self.state == SerState::WritingField) {
            return Err(Error::DTypeNotSupported(self.state.as_str()));
        }

        self.buf.push_byte(TokenTag::I8 as u8)?;
        self.buf.push_byte(v.to_be_bytes()[0])?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        if !(self.state == SerState::WritingField) {
            return Err(Error::DTypeNotSupported(self.state.as_str()));
        }

        self.buf.push_byte(TokenTag::I16 as u8)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        match &mut self.state {
            SerState::WritingField => self.buf.push_byte(TokenTag::I32 as u8)?,
            SerState::WrittingSeq(len) => {
                self.buf.push_byte(TokenTag::I32Arr as u8)?;
                self.buf.push_bytes(&(*len as u16).to_be_bytes())?;
                self.state = SerState::WrittingElement(TokenTag::I32 as u8);
            },
            SerState::WrittingElement(tk)  => {
                TokenTag::I32.matches(*tk)
                    .map_err(|_| Error::MissmatchTupleDType { expected: TokenTag::I32 as u8, found: *tk})?;
            }
            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        };

        self.buf.push_bytes(&v.to_be_bytes())?;

        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        match &mut self.state {
            SerState::WritingField => self.buf.push_byte(TokenTag::I64 as u8)?,
            SerState::WrittingSeq(len) => {
                self.buf.push_byte(TokenTag::I64Arr as u8)?;
                self.buf.push_bytes(&(*len as u16).to_be_bytes())?;
                self.state = SerState::WrittingFixedSeq;
            }
            SerState::WrittingElement(_) | SerState::WrittingTime => {},
            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        }
        
        self.buf.push_bytes(&v.to_be_bytes())?;

        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        match &mut self.state {
            SerState::WritingField => self.buf.push_byte(TokenTag::U8 as u8)?,
            SerState::WrittingSeq(len) => {
                self.buf.push_byte(TokenTag::U8Arr as u8)?;
                self.buf.push_bytes(&(*len as u16).to_be_bytes())?;
                self.state = SerState::WrittingElement(TokenTag::U8 as u8);
            }
            SerState::WrittingElement(tk)  => {
                TokenTag::U8.matches(*tk)
                    .map_err(|_| Error::MissmatchTupleDType { expected: TokenTag::U8 as u8, found: *tk})?;
            }
            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        }

        self.buf.push_byte(v)?;

        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        if !(self.state == SerState::WritingField) {
            return Err(Error::DTypeNotSupported(self.state.as_str()));
        }

        self.buf.push_byte(TokenTag::U16 as u8)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        if !(self.state == SerState::WritingField) {
            return Err(Error::DTypeNotSupported(self.state.as_str()));
        }

        self.buf.push_byte(TokenTag::U32 as u8)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        match self.state {
            SerState::WritingField => self.buf.push_byte(TokenTag::U64 as u8)?,
            SerState::WrittingTime => {}
            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        }

        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        match &mut self.state {
            SerState::WritingField => self.buf.push_byte(TokenTag::F32 as u8)?,
            SerState::WrittingSeq(len) => {
                self.buf.push_byte(TokenTag::F32Arr as u8)?;
                self.buf.push_bytes(&(*len as u16).to_be_bytes())?;
                self.state = SerState::WrittingElement(TokenTag::F32 as u8);
            }
            SerState::WrittingElement(tk)  => {
                TokenTag::F32.matches(*tk)
                    .map_err(|_| Error::MissmatchTupleDType { expected: TokenTag::F32 as u8, found: *tk})?;
            }
            SerState::WrittingFixedSeq => {}
            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        }

        self.buf.push_bytes(&v.to_be_bytes())?;

        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        match &mut self.state {
            SerState::WritingField => self.buf.push_byte(TokenTag::F64 as u8)?,
            SerState::WrittingSeq(len) => {
                self.buf.push_byte(TokenTag::F64Arr as u8)?;
                self.buf.push_bytes(&(*len as u16).to_be_bytes())?;
                self.state = SerState::WrittingElement(TokenTag::F64 as u8);
            }
            SerState::WrittingElement(tk)  => {
                TokenTag::F64.matches(*tk)
                    .map_err(|_| Error::MissmatchTupleDType { expected: TokenTag::F64 as u8, found: *tk})?;
            }
            SerState::WrittingFixedSeq => {},
            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        }
        
        self.buf.push_bytes(&v.to_be_bytes())?;

        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::SerUnsupported("char"))
    }

    fn serialize_str(self, v: &str) -> core::result::Result<Self::Ok, Self::Error> {
        match &mut self.state {
            SerState::WritingField => self.buf.push_byte(TokenTag::Str as u8)?,
            SerState::WrittingSeq(len) => {
                self.buf.push_byte(TokenTag::StrArr as u8)?;
                self.buf.push_bytes(&(*len as u16).to_be_bytes())?;
                self.state = SerState::WrittingElement(TokenTag::Str as u8);
            }
            SerState::WrittingElement(tk)  => {
                TokenTag::Str.matches(*tk)
                    .map_err(|_| Error::MissmatchTupleDType { expected: TokenTag::Str as u8, found: *tk})?;
            }
            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        }

        self.buf.push_bytes(&(v.len() as u16).to_be_bytes())?;
        self.buf.push_bytes(&v.as_bytes())?;
        
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.buf.push_byte(TokenTag::U8Arr as u8)?;
        self.buf.push_bytes(&v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        Err(Error::SerUnsupported("None"))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::SerUnsupported("Some"))
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Err(Error::SerUnsupported("unit ()"))
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok> {
        Err(Error::SerUnsupported("unit struct"))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok> {
        self.buf.push_byte(TokenTag::Enum as u8)?;
        self.buf.push_bytes(&(variant.len() as u16).to_be_bytes())?;
        self.buf.push_bytes(&variant.as_bytes())?;
        Ok(())
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        let token_tag = TokenTag::try_from(name)?;
        self.buf.push_byte(token_tag as u8)?;

        match token_tag {
            TokenTag::Vec2
             | TokenTag::Vec3
             | TokenTag::Vec4
             | TokenTag::Quat => self.state = SerState::WrittingFixedSeq,
            TokenTag::TimestampMillis
             | TokenTag::TimestampMicros
             | TokenTag::MillisSinceBoot
             | TokenTag::MicrosSinceBoot
             | TokenTag::DurationMillis
             | TokenTag::DurationMicros => self.state = SerState::WrittingTime,

            _ => return Err(Error::DTypeNotSupported(self.state.as_str()))
        }

        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::SerUnsupported("newtype variant"))
    }

    fn serialize_seq(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeSeq> {
        let len = len.ok_or(Error::LengthRequired)? as u16;

        self.state = SerState::WrittingSeq(len);

        if len == 0 {
            self.buf.push_byte(TokenTag::EmptyArr as u8)?;
        }

        Ok(SeqSerializer { ser: self })
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        if !(self.state == SerState::WrittingFixedSeq) {
            self.state = SerState::WrittingSeq(len as u16);
        }
        Ok(SeqSerializer { ser: self })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.buf.push_byte(TokenTag::Struct as u8)?;
        self.buf.push_byte(len as u8)?;
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.state = SerState::WrittingSeq(len as u16);
        Ok(SeqSerializer { ser: self })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::SerUnsupported("tuple variant"))
    }

    fn serialize_map(
        self,
        len: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        let len = len.ok_or(Error::LengthRequired)?;

        (TokenTag::U8 as u8).serialize(&mut *self)?;
        (len as u16).serialize(&mut *self)?;

        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.buf.push_byte(TokenTag::Enum as u8)?;
        self.buf.push_byte(u8::try_from(variant_index).map_err(|_| Error::IndexVariantExceeded)?)?;
        Ok(self)
    }
}

impl<'a, B: SerializerBuf> ser::SerializeSeq for SeqSerializer<'a, B> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeTuple for SeqSerializer<'a, B> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeTupleStruct for SeqSerializer<'a, B> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut *self.ser)
    }

    fn end(self) -> Result<Self::Ok> {
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeTupleVariant for &'a mut Serializer<B> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeMap for &'a mut Serializer<B> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeStruct for &'a mut Serializer<B> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.buf.push_bytes(&(key.len() as u16).to_be_bytes())?;
        self.buf.push_bytes(&key.as_bytes())?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeStructVariant for &'a mut Serializer<B> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

macro_rules! impl_serialize_time {
    ($Time:ident) => {
        impl serde::Serialize for $Time {
            fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
                {
                serializer.serialize_newtype_struct(stringify!($Time), &self.0)
            }
        }
    };
}

impl_serialize_time!(TimestampMillis);
impl_serialize_time!(TimestampMicros);
impl_serialize_time!(MillisSinceBoot);
impl_serialize_time!(MicrosSinceBoot);
impl_serialize_time!(DurationMillis);
impl_serialize_time!(DurationMicros);

impl_serialize_vec!(Vec2, x, y);
impl_serialize_vec!(Vec3, x, y, z);
impl_serialize_vec!(Vec4, x, y, z, w);
impl_serialize_vec!(Quat, x, y, z, w);