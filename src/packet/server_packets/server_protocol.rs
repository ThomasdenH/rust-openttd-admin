use crate::packet::read::OpenTTDRead;
use byteorder::{ReadBytesExt, LittleEndian};
use failure::Error;

/// Inform a just joined admin about the protocol specifics.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ServerProtocol {
    /// Protocol version.
    protocol_version: u8,
    packet_types: Vec<(u16, u16)>
}

impl ServerProtocol {
    pub fn from_buffer(mut buffer: &[u8]) -> Result<ServerProtocol, Error> {
        let protocol_version = buffer.read_u8()?;
        let mut packet_types = Vec::new();
        while buffer.read_bool()? {
            packet_types.push((buffer.read_u16::<LittleEndian>()?, buffer.read_u16::<LittleEndian>()?));
        }
        Ok(ServerProtocol {
            protocol_version,
            packet_types
        })
    }
}
