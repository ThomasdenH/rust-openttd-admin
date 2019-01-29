use std::io;
use std::io::Read;
use byteorder::{ByteOrder, LittleEndian, ReadBytesExt};

pub enum AdminServerPacket {

}

#[derive(Debug)]
pub enum ReadAdminServerPacketError {
    InvalidPacket,
    UnknownPacket
}

impl std::error::Error for ReadAdminServerPacketError {

}

pub trait ReadAdminServerPacket: io::Read {
    fn read_packet(&mut self) -> Result<AdminServerPacket, ReadAdminServerPacketError> {
        let length = self.read_u16::<LittleEndian>()?;
        let packet_type = self.read_u8()?;
        let buffer_length = length - 3;
        let mut buffer = Vec::with_capacity(buffer_length as usize);
        self.take(buffer_length as u64).read(&mut buffer);
        match packet_type {
            InvalidAdminPacket => Err(ReadAdminServerPacketError::InvalidPacket),
            _ => Err(ReadAdminServerPacketError::UnknownPacket)
        }
    }
}
