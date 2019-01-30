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
        Ok(match packet_type {
            100 => Full,
            101 => Banned,
            102 => Error,
            103 => Protocol,
            104 => Welcome,
            105 => Newgame,
            106 => Shutdown,
            107 => Date,
            108 => ClientJoin,
            109 => ClientInfo,
            110 => ClientUpdate,
            111 => ClientQuit,
            112 => ClientError,
            113 => CompanyNew,
            114 => CompanyInfo(crate::packet::serde::from_bytes(&buffer)?),
            115 => CompanyUpdate,
            116 => CompanyRemove,
            117 => CompanyEconomy,
            118 => CompanyStats,
            119 => Chat,
            120 => Rcon,
            121 => Console,
            122 => CmdNames,
            123 => CmdLogging,
            124 => Gamescript,
            125 => RconEnd,
            126 => Pong,
            _ => UnknownPacket { packet_type, buffer }
        })
    }
}

impl<R: std::io::Read + ?Sized> ReadAdminServerPacket for R {}
