use super::error::{Error, Result};
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use serde::ser::{self, Impossible, Serialize, SerializeSeq};
use std::io::Write;

struct Serializer {
    output: Vec<u8>,
    /// Indicates an option was serialized, if it was no more input is allowed.
    serialized_option: bool,
}

impl Serializer {
    /// Checks whether the serializer can accept more input and returns an
    /// error otherwise.
    fn check_can_serialize_more(&self) -> Result<()> {
        if self.serialized_option {
            Err(Error::InvalidOption)
        } else {
            Ok(())
        }
    }
}

pub trait WritablePacket: Serialize {
    const PACKET_TYPE: u8;
}

pub trait PacketWrite<T: WritablePacket> {
    fn write_packet(&mut self, value: &T) -> Result<()>;
}

impl<T: WritablePacket, W: std::io::Write> PacketWrite<T> for W {
    fn write_packet(&mut self, value: &T) -> Result<()> {
        let mut serializer = Serializer {
            output: vec![0, 0, T::PACKET_TYPE],
            serialized_option: false,
        };
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

    type SerializeTuple = Self;
    type SerializeSeq = Self;
    type SerializeTupleStruct = Self;
    type SerializeStruct = Self;

    type SerializeTupleVariant = Impossible<(), Error>;
    type SerializeMap = Impossible<(), Error>;
    type SerializeStructVariant = Impossible<(), Error>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_u8(if v { 1 } else { 0 })?;
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_i8(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_u8(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_i16::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_u16::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_i32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_u32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_i64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_u64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_f32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_f64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        // Serialize a char as a string.
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, s: &str) -> Result<()> {
        self.check_can_serialize_more()?;
        self.output.write_all(s.as_bytes())?;
        self.output.write_u8(0)?;
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.check_can_serialize_more()?;
        // Serialize bytes as a u8 sequence.
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for b in v {
            seq.serialize_element(b)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        // None is denoted by EOF. Because of this, we don't write anything,
        // but we need to make sure nothing more is written.
        self.serialized_option = true;
        Ok(())
    }

    fn serialize_some<T>(self, v: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Since None is denoted by EOF, we serialize Some by just writing the
        // inner data. We do set the flag to make sure nothing more is written.
        self.serialized_option = true;
        v.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        // An unit variant should provide their own implementation since the
        // serialized size could differ.
        Err(Error::NotSupported)
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        // Just serialize the inner data.
        value.serialize(self)
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
        // A variant should provide their own implementation since the
        // serialized size could differ.
        Err(Error::NotSupported)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        // There is no obvious default way to implement a map.
        Err(Error::NotSupported)
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        // There is no obvious default implementation here.
        Err(Error::NotSupported)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Ok(self)
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

/// A sequence is serialized by prefixing each element with `true` and ending with `false`.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        true.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    // Close the sequence.
    fn end(self) -> Result<()> {
        false.serialize(self)
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

/// Implement structs by serializing the contents in order.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
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

/// To serialize a tuple struct, simply output the elements since the size is known.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
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

#[cfg(test)]
mod test {
    use super::*;
    use serde_derive::Serialize;

    #[test]
    fn test_simple_struct_ser() {
        #[derive(Serialize)]
        struct SimpleStruct {
            a: u8,
            b: u16,
            c: u32,
            d: bool
        }
        impl WritablePacket for SimpleStruct {
            const PACKET_TYPE: u8 = 10;
        }
        let simple_struct = SimpleStruct { a: 1, b: 2, c: 3, d: true};
        let mut buffer = Vec::new();
        let output = &mut buffer;
        output.write_packet(&simple_struct).unwrap();
        assert_eq!(buffer, vec![
            11, 0, // Length
            10, // PACKET_TYPE
            1, // a
            2, 0, // b
            3, 0, 0, 0, // c
            1 // d
        ]);
    }
}
