//! Provides API for communicating with the OpenTTD admin interface. To read
//! packets, import [`AdminRead`](crate::packet::admin::AdminRead), which provides [`read_packet`](crate::packet::admin::AdminRead#read_packet).
//! To write packets, import [`AdminWrite`](crate::packet::admin::AdminWrite), which provides [`write_packet`](crate::packet::admin::AdminRead#write_packet).
//! The packets themselves are defined in the submodules [`client_packets`](crate::packet::admin::client_packets) and
//! [`server_packets`](crate::packet::admin::server_packets).

pub mod client_packets;
pub mod server_packets;

pub use crate::packet::serde::{PacketRead, PacketWrite, Result};

/// Provides the function [`AdminRead::read_packet`]. It is implemented for any type implementing std::io::Read via PacketRead.
pub trait AdminRead {
    fn read_packet(&mut self) -> Result<server_packets::Packet>;
}

impl<T: PacketRead> AdminRead for T {
    fn read_packet(&mut self) -> Result<server_packets::Packet> {
        let (packet_type, buffer) = PacketRead::read_packet(self)?;
        use crate::packet::serde::from_bytes;
        use server_packets::Packet::*;
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
            _ => UnknownPacket {
                packet_type,
                buffer,
            },
        })
    }
}

/// Provides the function [`AdminWrite::write_packet`] to a type implementing
/// [`std::io::Write`].
pub trait AdminWrite<T: client_packets::Packet> {
    fn write_packet(&mut self, value: &T) -> Result<()>;
}

impl<T: client_packets::Packet, W: PacketWrite<T>> AdminWrite<T> for W {
    fn write_packet(&mut self, value: &T) -> Result<()> {
        self.write_packet(value)
    }
}
