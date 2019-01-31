//! Provides API for communicating with the OpenTTD admin interface. To read
//! packets, import [`AdminRead`], which provides [`AdminRead::read_packet`].
//! To write packets, import [`AdminWrite`], which provides [`AdminWrite::write_packet`].
//! The packets themselves are defined in the submodules [`client_packets`] and
//! [`server_packets`].

pub mod client_packets;
pub mod server_packets;

pub use crate::packet::serde::{PacketWrite, PacketRead, Result};

/// Provides the function [`AdminRead::read_packet`] to a type implementing
/// [`std::io::Read`].
pub trait AdminRead: std::io::Read { }
impl<T: std::io::Read> AdminRead for T { }
impl<'a, T: AdminRead> PacketRead<'a> for T {
    type PACKET_TYPE = server_packets::Packet;
    fn match_packet(packet_type: u8, buffer: Vec<u8>) -> Result<Self::PACKET_TYPE> {
        use server_packets::Packet::*;
        use crate::packet::serde::from_bytes;
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

/// Provides the function [`AdminWrite::write_packet`] to a type implementing
/// [`std::io::Write`].
pub trait AdminWrite: std::io::Write { }
impl<T: std::io::Write> AdminWrite for T { }
impl<T: client_packets::Packet, W: AdminWrite> PacketWrite<T> for W { }
