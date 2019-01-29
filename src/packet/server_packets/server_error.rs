use byteorder::{ReadBytesExt};
use failure::Error;

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct ServerError {
    /// The error caused.
    network_error_code: u8
}

impl ServerError {
    pub fn from_buffer(mut buffer: &[u8]) -> Result<ServerError, Error> {
        Ok(ServerError {
            network_error_code: buffer.read_u8()?
        })
    }
}
