use byteorder::{LittleEndian, ReadBytesExt};
use failure::{Error, Fail};
use num_traits::{FromPrimitive};
use crate::consts::PacketAdminServerType;
use std::io::{BufReader, BufRead};
use crate::packet::server_packets;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum AdminServerPacket {
    ServerFull,
    ServerBanned,
    CompanyInfo(server_packets::CompanyInfo),
    ServerError(server_packets::ServerError),
    ServerProtocol(server_packets::ServerProtocol)
}

#[derive(Debug, Fail)]
pub enum ReadAdminServerPacketError {
    #[fail(display = "unknown packet type: {}", packet_type)]
    UnknownPacket { packet_type: u8 },
}

pub trait OpenTTDRead: std::io::Read {
    fn read_bool(&mut self) -> std::io::Result<bool> {
        let mut buffer = [0u8];
        self.read(&mut buffer)?;
        Ok(buffer[0] == 1)
    }

    fn read_string(&mut self) -> Result<String, Error> {
        let mut f = BufReader::new(self);
        let mut buffer = Vec::new();
        f.read_until(0, &mut buffer)?;
        Ok(String::from_utf8(buffer)?)
    }
}

impl<R: std::io::Read + ?Sized> OpenTTDRead for R {}

pub trait ReadAdminServerPacket: std::io::Read {
    fn read_packet(&mut self) -> Result<AdminServerPacket, Error> {
        let length = self.read_u16::<LittleEndian>()? as usize;
        let packet_type = self.read_u8()?;
        let buffer_length = length - 3;
        let mut buffer = vec![0u8; buffer_length];
        self.read_exact(&mut buffer)?;
        if let Some(packet_type) = PacketAdminServerType::from_u8(packet_type) {
            Ok(match packet_type {
                PacketAdminServerType::ServerFull => AdminServerPacket::ServerFull,
                PacketAdminServerType::ServerBanned => AdminServerPacket::ServerBanned,
                PacketAdminServerType::ServerCompanyInfo => AdminServerPacket::CompanyInfo(server_packets::CompanyInfo::from_buffer(&mut buffer)?),
                PacketAdminServerType::ServerError => AdminServerPacket::ServerError(server_packets::ServerError::from_buffer(&mut buffer)?),
                PacketAdminServerType::ServerProtocol => AdminServerPacket::ServerProtocol(server_packets::ServerProtocol::from_buffer(&mut buffer)?),
            })
        } else {
            Err(ReadAdminServerPacketError::UnknownPacket { packet_type }.into())
        }
    }
}

impl<R: std::io::Read + ?Sized> ReadAdminServerPacket for R {}
