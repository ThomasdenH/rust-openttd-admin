use super::error::{Error, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use serde::de::{self, Deserialize, DeserializeSeed, SeqAccess, Visitor};

struct Deserializer<'de> {
    input: &'de [u8],
}

impl<'de> Deserializer<'de> {
    fn from_bytes(input: &'de [u8]) -> Self {
        Deserializer { input }
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        if let Some(end) = self.input.iter().position(|&b| b == 0) {
            let string = &self.input[0..end];
            self.input = &self.input[end + 1..];
            let string = &std::str::from_utf8(&string)?;
            Ok(string)
        } else {
            Err(Error::EndlessString)
        }
    }

    fn parse_bool(&mut self) -> Result<bool> {
        let b = self.input.read_u8()?;
        match b {
            1 => Ok(true),
            0 => Ok(false),
            _ => Err(Error::InvalidBool),
        }
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

/// A trait that provides the [`read_packet`](PacketRead#read_packet) function
/// to a type implementing [`std::io::Read`].
pub trait PacketRead {
    /// Read a packet and return the packet type and data.
    fn read_packet(&mut self) -> Result<(u8, Vec<u8>)>;
}

impl<T: std::io::Read> PacketRead for T {
    fn read_packet(&mut self) -> Result<(u8, Vec<u8>)> {
        let length = self.read_u16::<LittleEndian>()? as usize;
        let packet_type = self.read_u8()?;
        let buffer_length = length - 3;
        let mut buffer = vec![0u8; buffer_length];
        self.read_exact(&mut buffer)?;
        Ok((packet_type, buffer))
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // We really have no idea what the data means.
        Err(Error::NotSupported)
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = self.parse_string()?;
        if string.chars().count() != 1 {
            Err(Error::InvalidChar)
        } else {
            visitor.visit_char(string.chars().next().unwrap())
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(self.parse_bool()?)
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
        let string = self.parse_string()?;
        visitor.visit_borrowed_str(string)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let string = self.parse_string()?;
        visitor.visit_borrowed_str(string)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.input.is_empty() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(self)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FixedSizeSeqAccess { de: self, len })
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FixedSizeSeqAccess { de: self, len })
    }

    fn deserialize_map<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Maps are not supported
        Err(Error::NotSupported)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_seq(FixedSizeSeqAccess {
            de: self,
            len: fields.len(),
        })
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Use custom implementation instead.
        Err(Error::NotSupported)
    }

    fn deserialize_identifier<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::NotSupported)
    }
}

impl<'de> SeqAccess<'de> for Deserializer<'de> {
    type Error = Error;
    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let expect_next = self.parse_bool()?;
        if expect_next {
            seed.deserialize(self).map(Some)
        } else {
            Ok(None)
        }
    }
}

/// A SeqAccess of fixed size without delimiters.
struct FixedSizeSeqAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    len: usize,
}

impl<'de, 'a> SeqAccess<'de> for FixedSizeSeqAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.len == 0 {
            Ok(None)
        } else {
            self.len -= 1;
            seed.deserialize(&mut *self.de).map(Some)
        }
    }
}

#[cfg(test)]
mod test {
    use super::PacketRead;

    #[test]
    fn test_empty_packet_read() {
        let mut empty_packet: &[u8] = &[3, 0, 10];
        assert_eq!(empty_packet.read_packet().unwrap(), (10, Vec::new()));
    }

    use super::*;
    use serde_derive::Deserialize;

    #[test]
    fn test_simple_struct_read() {
        #[derive(Deserialize, Eq, PartialEq, Debug)]
        struct SimpleStruct {
            a: u8,
            b: u16,
            c: u32,
            d: bool
        }
        let mut input: &[u8] = &vec![
            11, 0, // Length
            10, // PACKET_TYPE
            1, // a
            2, 0, // b
            3, 0, 0, 0, // c
            1 // d
        ];
        let simple_struct = SimpleStruct { a: 1, b: 2, c: 3, d: true};
        let (packet_type, buffer) = input.read_packet().unwrap();
        assert_eq!(packet_type, 10);
        assert_eq!(from_bytes::<SimpleStruct>(&buffer).unwrap(), simple_struct);
    }
}
