/// A serializer and deserializer for the OpenTTD packet format.

mod de;
mod error;
mod ser;

pub use error::{Error, Result};
pub use ser::{to_bytes, Serializer};
pub use de::{from_bytes, Deserializer};
