use num_derive::{FromPrimitive, ToPrimitive};
use num_traits::{FromPrimitive, ToPrimitive};
use serde::de::{Deserialize, Unexpected};
use serde::{de, ser, Deserializer, Serialize, Serializer};

/// Update types an admin can register a frequency for
#[derive(FromPrimitive, ToPrimitive, Copy, Clone, Eq, PartialEq, Debug)]
pub enum AdminUpdateType {
    /// Updates about the date of the game.
    Date,
    /// Updates about the information of clients.    
    ClientInfo,
    /// Updates about the generic information of companies.
    CompanyInfo,
    /// Updates about the economy of companies.
    CompanyEconomy,
    /// Updates about the statistics of companies.
    CompanyStats,
    /// The admin would like to have chat messages.
    Chat,
    /// The admin would like to have console messages.
    Console,
    /// The admin would like a list of all DoCommand names.
    CmdNames,
    /// The admin would like to have DoCommand information.
    CmdLogging,
    /// The admin would like to have gamescript messages.
    Gamescript,
}

const ADMIN_UPDATE_TYPE_SERIALIZE_ERROR: &str = "could not serialze AdminUpdateType";

impl Serialize for AdminUpdateType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_u16()
            .ok_or_else(|| ser::Error::custom(ADMIN_UPDATE_TYPE_SERIALIZE_ERROR))
            .and_then(|num| serializer.serialize_u16(num))
    }
}

impl<'de> Deserialize<'de> for AdminUpdateType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        u16::deserialize(deserializer).and_then(|num| {
            AdminUpdateType::from_u16(num).ok_or_else(|| {
                de::Error::invalid_value(
                    Unexpected::Unsigned(u64::from(num)),
                    &"a variant of AdminUpdateType",
                )
            })
        })
    }
}
