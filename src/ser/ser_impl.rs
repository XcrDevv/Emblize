use crate::{
    core::{token::TokenTag, types::*}, 
    error::{Error, Result}, 
    impl_serialize_vec, 
    ser::serializer::{SerState, Serializer, SerializerBuf}
};
use serde::{
    Serialize,
    ser::{self},
};

pub fn serialize<T, B: SerializerBuf>(value: &T) -> Result<B>
where
    T: Serialize + ?Sized,
{
    let mut serializer: Serializer<B> = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.buf)
}

pub struct SeqSerializer<'a, B: SerializerBuf> {
    ser: &'a mut Serializer<B>,
    wrote_elem_type: bool,
    prev_state: SerState,
}

pub struct TupSerializer<'a, B: SerializerBuf> {
    ser: &'a mut Serializer<B>,
    first_tag: Option<TokenTag>,
    prev_state: SerState
}

impl<'a, B: SerializerBuf> Serializer<B> {
    fn write_tag(&mut self, token: TokenTag) -> Result<()> {
        match self.state {
            SerState::WriteTypedValue => {
                self.buf.push_byte(token as u8)?;
            },
            SerState::WriteSeqHeader | SerState::WriteVecHeader => {
                self.buf.push_byte(token as u8)?;
            },
            SerState::WriteUntypedChecked(expected) => {
                if expected != token {
                    return Err(Error::HeterogeneousTuple {
                        expected: expected as u8,
                        got: token as u8,
                    });
                }
            },
            _ => {}
        }

        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::Serializer for &'a mut Serializer<B> {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = SeqSerializer<'a, B>;
    type SerializeTuple = TupSerializer<'a, B>;
    type SerializeTupleStruct = SeqSerializer<'a, B>;
    type SerializeTupleVariant = TupSerializer<'a, B>;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok> {
        self.write_tag(TokenTag::Bool)?;
        self.buf.push_byte(if v { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.write_tag(TokenTag::I8)?;
        self.buf.push_byte(v.to_be_bytes()[0])?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.write_tag(TokenTag::I16)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.write_tag(TokenTag::I32)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.write_tag(TokenTag::I64).unwrap();
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.write_tag(TokenTag::U8)?;
        self.buf.push_byte(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.write_tag(TokenTag::U16)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.write_tag(TokenTag::U32)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.write_tag(TokenTag::U64).unwrap();
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.write_tag(TokenTag::F32)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.write_tag(TokenTag::F64)?;
        self.buf.push_bytes(&v.to_be_bytes())?;
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::SerUnsupported("char"))
    }

    fn serialize_str(self, v: &str) -> core::result::Result<Self::Ok, Self::Error> {
        self.write_tag(TokenTag::Str)?;
        self.write_varint_usize(v.len())?;
        self.buf.push_bytes(&v.as_bytes())?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok> {
        self.write_tag(TokenTag::Bytes)?;
        self.write_varint_usize(v.len())?;
        self.buf.push_bytes(&v)?;
        Ok(())
    }

    fn serialize_none(self) -> Result<Self::Ok> {
        if let SerState::WriteSeqHeader = self.state {
            self.buf.push_byte(TokenTag::None as u8)?;
        }

        self.buf.push_byte(TokenTag::None as u8)?;
        Ok(())
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        if let SerState::WriteSeqHeader = self.state {
            self.buf.push_byte(TokenTag::Some as u8)?;
            self.state = SerState::WriteUntypedValue;
        }

        self.buf.push_byte(TokenTag::Some as u8)?;

        let prev = self.state;
        if matches!(self.state, SerState::WriteUntypedChecked(_)) {
            self.state = SerState::WriteUntypedValue;
        }
        value.serialize(&mut *self)?;
        self.state = prev;
        Ok(())
    }

    fn serialize_unit(self) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_struct(
        self,
        _name: &'static str,
    ) -> Result<Self::Ok> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok> {
        self.write_tag(TokenTag::Enum)?;
        if variant_index > 0x7F {
            Err(Error::IndexVariantExceeded)
        } else {
            self.buf.push_byte(variant_index as u8)?;
            Ok(())
        }
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
        
        self.write_tag(token_tag)?;
        match token_tag {
            TokenTag::Vec2
             |TokenTag::Vec3
             |TokenTag::Vec4
             |TokenTag::Quat => self.state = SerState::WriteVecHeader,
            _ => self.state = SerState::WriteUntypedValue
        }

        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok>
    where
        T: ?Sized + Serialize,
    {
        self.state = SerState::WriteTypedValue;
        self.write_tag(TokenTag::Enum)?;
        if variant_index > 0x7F {
            Err(Error::IndexVariantExceeded)
        } else {
            self.buf.push_byte(variant_index as u8 | 0x80)?;
            value.serialize(self)
        }
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq> {
        let len = len.ok_or(Error::LengthRequired)?;
        let prev_state = self.state;

        if len == 0 {
            self.write_tag(TokenTag::EmptyArr)?;
            return Ok(SeqSerializer {
                ser: self,
                wrote_elem_type: true, 
                prev_state,
            });
        }

        self.write_tag(TokenTag::Array)?;
        self.write_varint_usize(len)?;
        self.state = SerState::WriteSeqHeader;

        Ok(SeqSerializer { ser: self, wrote_elem_type: false, prev_state })
    }


fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
    let prev_state = self.state;

    if self.state != SerState::WriteVecHeader {
        self.write_tag(TokenTag::Array)?;
        self.write_varint_usize(len)?;
        self.state = SerState::WriteSeqHeader;
    }

    Ok(TupSerializer {
        prev_state,
        ser: self,
        first_tag: None,
    })
}

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct> {
        self.write_tag(TokenTag::Struct)?;
        self.write_varint_usize(len)?;
        Ok(self)
    }
    
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.state = SerState::WriteSeqHeader;
        Ok(SeqSerializer { prev_state: self.state, ser: self, wrote_elem_type: false })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.write_tag(TokenTag::Enum)?;
        if variant_index > 0x7F {
            Err(Error::IndexVariantExceeded)
        } else {
            self.state = SerState::WriteSeqHeader;
            self.buf.push_byte(variant_index as u8 | 0x80)?;
            let prev_state = self.state;

            if self.state != SerState::WriteVecHeader {
                self.write_tag(TokenTag::Array)?;
                self.write_varint_usize(len)?;
                self.state = SerState::WriteSeqHeader;
            }

            Ok(TupSerializer {
                prev_state,
                ser: self,
                first_tag: None,
            })
        }
    }

    fn serialize_map(
        self,
        _len: Option<usize>,
    ) -> Result<Self::SerializeMap> {
        unimplemented!("Map serialization has not yet been implemented") // ! CHECK
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.write_tag(TokenTag::Enum)?;
        if variant_index > 0x7F {
            Err(Error::IndexVariantExceeded)
        } else {
            self.buf.push_byte(variant_index as u8 | 0x80)?;
            self.buf.push_byte(TokenTag::Struct as u8)?;
            self.write_varint_usize(len)?;
            Ok(self)
        }
    }
}

impl<'a, B: SerializerBuf> ser::SerializeSeq for SeqSerializer<'a, B> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        if !self.wrote_elem_type {
            self.ser.state = SerState::WriteSeqHeader;
            value.serialize(&mut *self.ser)?;
            self.wrote_elem_type = true;
        } else {
            self.ser.state = SerState::WriteUntypedValue;
            value.serialize(&mut *self.ser)?;
        }
        Ok(())
    }

    fn end(self) -> Result<Self::Ok> {
        self.ser.state = self.prev_state;
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeTuple for TupSerializer<'a, B> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {

        match self.first_tag {
            None => {
                let pos_before = self.ser.buf.as_slice().len();
                value.serialize(&mut *self.ser)?;
                let written_tag = self.ser.buf.as_slice()[pos_before];
                self.first_tag = Some(TokenTag::try_from(written_tag).unwrap());
            }
            Some(expected) => {
                self.prev_state = self.ser.state;
                if self.ser.state == SerState::WriteVecHeader {
                    self.ser.state = SerState::WriteUntypedValue;
                } else {
                    self.ser.state = SerState::WriteUntypedChecked(expected);
                }
                value.serialize(&mut *self.ser)?;
            }
        }
        Ok(())
    }


    fn end(self) -> Result<Self::Ok> {
        self.ser.state = match self.prev_state {
            SerState::WriteSeqHeader => SerState::WriteUntypedValue,
            other => other,
        };
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeTupleVariant for TupSerializer<'a, B> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {

        match self.first_tag {
            None => {
                let pos_before = self.ser.buf.as_slice().len();
                value.serialize(&mut *self.ser)?;
                let written_tag = self.ser.buf.as_slice()[pos_before];
                self.first_tag = Some(TokenTag::try_from(written_tag).unwrap());
            }
            Some(expected) => {
                self.prev_state = self.ser.state;
                if self.ser.state == SerState::WriteVecHeader {
                    self.ser.state = SerState::WriteUntypedValue;
                } else {
                    self.ser.state = SerState::WriteUntypedChecked(expected);
                }
                value.serialize(&mut *self.ser)?;
            }
        }
        Ok(())
    }


    fn end(self) -> Result<Self::Ok> {
        self.ser.state = match self.prev_state {
            SerState::WriteSeqHeader => SerState::WriteUntypedValue,
            other => other,
        };
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
        let temp = self.state;
        self.write_varint_usize(key.len())?;
        self.buf.push_bytes(&key.as_bytes())?;
        self.state = SerState::WriteTypedValue;
        value.serialize(&mut **self)?;
        self.state = temp;
        Ok(())
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

impl<'a, B: SerializerBuf> ser::SerializeStructVariant for &'a mut Serializer<B> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.write_varint_usize(key.len())?;
        self.buf.push_bytes(&key.as_bytes())?;
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

impl serde::Serialize for Quat {
        fn serialize<S>(&self, serializer: S) -> core::result::Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            serializer.serialize_newtype_struct(
                "Quat",
                &(self.x, self.y, self.z, self.w)
            )
        }
    }