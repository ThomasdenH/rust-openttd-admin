//! This module contains specialized OpenTTD types.

mod admin_update_frequency;
mod admin_update_type;
mod date;

pub use admin_update_frequency::UpdateFrequencies;
pub use admin_update_type::AdminUpdateType;
pub use date::Date;
