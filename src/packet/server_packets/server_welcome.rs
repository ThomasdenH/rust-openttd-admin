use crate::packet::read::OpenTTDRead;
use byteorder::{ReadBytesExt};
use failure::Error;

/// Welcome a connected admin to the game.
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ServerWelcome {
    server_name: String,
    version: String,
    dedicated: bool,
    map_name: String,
    map_seed: u32,
    map_landscape: u8,
    map_start_date: u32,
    map_width: u16,
    map_height: u16
}

impl ServerWelcome {
    pub fn from_buffer(mut buffer: &[u8]) -> Result<ServerWelcome, Error> {
        Ok(ServerWelcome {
            company_index: buffer.read_u8()?,
            company_name: buffer.read_string()?,
            manager_name: buffer.read_string()?,
            colour: buffer.read_u8()?,
            is_ai: buffer.read_bool()?,
            quarters_of_bankruptcy: buffer.read_u8()?,
            shareowners: buffer.to_vec()
        })
    }
}
