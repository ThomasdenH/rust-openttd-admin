use serde::ser::{self, Serialize, Impossible};
use super::error::{Error, Result};
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use std::io::Write;

pub struct Serializer {
    output: Vec<u8>
}

pub trait WritablePacket: Serialize {
    const PACKET_TYPE: u8;
}

pub trait PacketWrite<T: WritablePacket>: std::io::Write {
    fn write_packet(&mut self, value: &T) -> Result<()> {
        let mut serializer = Serializer { output: vec![0, 0, T::PACKET_TYPE] };
        value.serialize(&mut serializer)?;
        let length = serializer.output.len() as u16;
        LittleEndian::write_u16(&mut serializer.output[0..2], length);
        self.write_all(&serializer.output)?;
        Ok(())
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeTuple = Impossible<(), Error>;
    type SerializeSeq = Impossible<(), Error>;
    type SerializeTupleStruct = Impossible<(), Error>;
    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStruct = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.write_u8(if v { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.write_i8(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.write_u8(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output.write_i16::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output.write_u16::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output.write_i32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output.write_u32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.write_i64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.write_u64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.output.write_f32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.write_f64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_char(self, _v: char) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn serialize_str(self, s: &str) -> Result<()> {
        self.output.write(s.as_bytes())?;
        self.output.write_u8(0)?;
        Ok(())
    }

    fn serialize_bytes(self, _b: &[u8]) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn serialize_some<T>(self, v: &T) -> Result<()> where
        T: ?Sized + Serialize
    {
        Err(Error::NotSupported)
    }

    fn serialize_unit(self) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::NotSupported)
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::NotSupported)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::NotSupported)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(Error::NotSupported)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(Error::NotSupported)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::NotSupported)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct> {
        Err(Error::NotSupported)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::NotSupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(Error::NotSupported)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::NotSupported)
    }
}

/// To serialize a tuple, simply output the elements since the size is known.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}
