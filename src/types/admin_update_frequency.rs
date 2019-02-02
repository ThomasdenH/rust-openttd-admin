use bitflags::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

bitflags! {
    pub struct UpdateFrequencies: u16 {
        const Poll = 0x01;
        const Daily = 0x02;
        const Weekly = 0x04;
        const Monthly = 0x08;
        const Quarterly = 0x10;
        const Annually = 0x20;
        const Automatic = 0x40;
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
