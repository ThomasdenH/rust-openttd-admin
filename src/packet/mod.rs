pub mod serde;
pub mod admin_server_packets;
pub mod admin_client_packets;

use failure::{Fail, Error};
use admin_server_packets::AdminServerPacket;
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug, Fail)]
pub enum ReadAdminServerPacketError {
    #[fail(display = "unknown packet type: {}", packet_type)]
    UnknownPacket { packet_type: u8 },
}

/// Read a packet from a source that implements [std::io::Read].
pub trait ReadAdminServerPacket: std::io::Read {
    fn read_packet(&mut self) -> Result<AdminServerPacket, Error> {
        let length = self.read_u16::<LittleEndian>()? as usize;
        let packet_type = self.read_u8()?;
        let buffer_length = length - 3;
        let mut buffer = vec![0u8; buffer_length];
        self.read_exact(&mut buffer)?;

        use AdminServerPacket::*;
        use self::serde::from_bytes;
        Ok(match packet_type {
            100 => Full,
            101 => Banned,
            102 => Error(from_bytes(&buffer)?),
            103 => Protocol(from_bytes(&buffer)?),
            104 => Welcome(from_bytes(&buffer)?),
            105 => Newgame,
            106 => Shutdown,
            107 => Date(from_bytes(&buffer)?),
            108 => ClientJoin(from_bytes(&buffer)?),
            109 => ClientInfo(from_bytes(&buffer)?),
            110 => ClientUpdate(from_bytes(&buffer)?),
            111 => ClientQuit(from_bytes(&buffer)?),
            112 => ClientError(from_bytes(&buffer)?),
            113 => CompanyNew(from_bytes(&buffer)?),
            114 => CompanyInfo(from_bytes(&buffer)?),
            115 => CompanyUpdate(from_bytes(&buffer)?),
            116 => CompanyRemove(from_bytes(&buffer)?),
            117 => CompanyEconomy(from_bytes(&buffer)?),
            118 => CompanyStats(from_bytes(&buffer)?),
            119 => Chat(from_bytes(&buffer)?),
            120 => Rcon(from_bytes(&buffer)?),
            121 => Console(from_bytes(&buffer)?),
            122 => CmdNames(from_bytes(&buffer)?),
            123 => CmdLogging(from_bytes(&buffer)?),
            124 => Gamescript(from_bytes(&buffer)?),
            125 => RconEnd(from_bytes(&buffer)?),
            126 => Pong(from_bytes(&buffer)?),
            _ => UnknownPacket { packet_type, buffer }
        })
    }
}

impl<R: std::io::Read + ?Sized> ReadAdminServerPacket for R {}
