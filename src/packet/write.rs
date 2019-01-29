use crate::consts::PacketAdminType;
use byteorder::{ByteOrder, LittleEndian};

/// Helps to write data to a OpenTTD packet.
pub struct PacketWriter {
    buffer: Vec<u8>
}

impl PacketWriter {
    pub fn create(packet_type: PacketAdminType) -> PacketWriter {
        PacketWriter {
            buffer: vec![0, 0, packet_type as u8]
        }
    }

    pub fn write_string(&mut self, string: &str) {
        self.buffer.write(string.as_bytes());
        self.buffer.write(0);
    }

    pub fn write_u8(&mut self, v: u8) {
        self.buffer.write(v);
    }

    pub fn write_u16(&mut self, v: u16) {
        self.buffer.write(v);
    }

    pub fn write_u32(&mut self, v: u32) {
        self.buffer.write(v);
    }

    pub fn build(mut self) -> Vec<u8> {
        let length = self.buffer.len() as u16;
        LittleEndian::write_u16(self.buffer.as_mut_slice(), length);
        self.buffer
    }
}
