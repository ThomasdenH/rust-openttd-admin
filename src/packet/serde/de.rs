use serde::de::{self, Deserialize, Visitor};
use super::error::{Error, Result};
use byteorder::{LittleEndian, ReadBytesExt};

struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input }
    }
}

/// Decode a serializable type from an OpenTTD buffer. The input should be the
/// data buffer, without the preceding length and packet type. Usually this is
/// is used to implement [`PacketRead`].
pub fn from_bytes<'b, T>(s: &'b [u8]) -> Result<T>
where
    T: Deserialize<'b>,
{
    let mut deserializer = Deserializer::from_bytes(s);
    let t = T::deserialize(&mut deserializer)?;
    if deserializer.input.is_empty() {
        Ok(t)
    } else {
        Err(Error::TrailingCharacters)
    }
}

/// A trait that provides the [`read_packet`] function to a type implementing
/// [`std::io::Read`].
pub trait PacketRead<'a>: std::io::Read {
    /// The type of the decoded packet.
    type PACKET_TYPE: Deserialize<'a>;

    /// Reads a packet from the source.
    fn read_packet(&mut self) -> Result<Self::PACKET_TYPE> {
        let length = self.read_u16::<LittleEndian>()? as usize;
        let packet_type = self.read_u8()?;
        let buffer_length = length - 3;
        let mut buffer = vec![0u8; buffer_length];
        self.read_exact(&mut buffer)?;
        Self::match_packet(packet_type, buffer)
    }

    /// Take a packet type and a data buffer and return a parsed packet.
    fn match_packet(packet_type: u8, buffer: Vec<u8>) -> Result<Self::PACKET_TYPE>;
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let b = self.input.read_u8()?;
        match b {
            1 => visitor.visit_bool(true),
            0 => visitor.visit_bool(false),
            _ => Err(Error::InvalidBool)
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i8(self.input.read_i8()?)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u8(self.input.read_u8()?)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i16(self.input.read_i16::<LittleEndian>()?)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u16(self.input.read_u16::<LittleEndian>()?)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i32(self.input.read_i32::<LittleEndian>()?)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u32(self.input.read_u32::<LittleEndian>()?)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.input.read_i64::<LittleEndian>()?)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.input.read_u64::<LittleEndian>()?)
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f32(self.input.read_f32::<LittleEndian>()?)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.input.read_f64::<LittleEndian>()?)
    }
    
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        for i in 0..self.input.len() {
            if self.input[i] == 0 {
                let string = &self.input[0..i];
                self.input = &self.input[i + 1..];
                return visitor.visit_borrowed_str(&std::str::from_utf8(&string)?);
            }
        }
        Err(Error::EndlessString)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_seq<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_map<V>(mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }
}
