use bitflags::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    pub struct UpdateFrequencies: u16 {
        const POLL = 0x01;
        const DAILY = 0x02;
        const WEEKLY = 0x04;
        const MONTHLY = 0x08;
        const QUARTERLY = 0x10;
        const ANNUALLY = 0x20;
        const AUTOMATIC = 0x40;
    }
}

impl Serialize for UpdateFrequencies {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u16(self.bits())
    }
}

impl<'de> Deserialize<'de> for UpdateFrequencies {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Self::from_bits(u16::deserialize(deserializer)?)
            .ok_or_else(|| serde::de::Error::custom("unknown update frequency"))
    }
}
