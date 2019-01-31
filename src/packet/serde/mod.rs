//! This module handles the serializing and deserializing of OpenTTD packets.
//!
//! It is designed to make most things go right by default, but it is far from
//! general purpose.

mod de;
mod error;
mod ser;

pub use de::{from_bytes, PacketRead};
pub use error::{Error, Result};
pub use ser::{PacketWrite, WritablePacket};
