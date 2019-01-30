/// A serializer and deserializer for the OpenTTD packet format.

mod de;
mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{BuildablePacket, Serializer};
pub use de::{from_bytes, Deserializer};
