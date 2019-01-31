/// A serializer and deserializer for the OpenTTD packet format.

mod de;
mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{PacketWrite, WritablePacket};
pub use de::{PacketRead, from_bytes};
