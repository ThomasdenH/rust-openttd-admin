use crate::consts::PacketAdminClientType;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use std::io::Write;
use num_traits::ToPrimitive;

/// Helps to write data to a OpenTTD packet.
pub struct PacketWriter {
    buffer: Vec<u8>
}

impl PacketWriter {
    pub fn new(packet_type: PacketAdminClientType) -> PacketWriter {
        PacketWriter {
            buffer: vec![0, 0, packet_type.to_u8().unwrap()]
        }
    }

    pub fn write_string(&mut self, string: &str) {
        self.buffer.write(string.as_bytes()).unwrap();
        self.buffer.write_u8(0).unwrap();
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buffer.write_u8(v).unwrap();
    }

    pub fn write_u16(&mut self, v: u16) {
        self.buffer.write_u16::<LittleEndian>(v).unwrap();
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buffer.write_u32::<LittleEndian>(v).unwrap();
    }

    pub fn build(mut self) -> Vec<u8> {
        let length = self.buffer.len() as u16;
        LittleEndian::write_u16(self.buffer.as_mut_slice(), length);
        self.buffer
    }
}
