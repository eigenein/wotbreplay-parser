#![cfg(feature = "meta")]

use std::io::Read;

use serde::Deserialize;

use crate::error::Error;
use crate::result::Result;

/// The model for `meta.json`.
#[derive(Deserialize)]
pub struct Meta {
    #[serde(rename = "playerName")]
    pub player_name: String,

    #[serde(rename = "arenaUniqueId")]
    pub arena_unique_id: String,

    #[serde(rename = "battleDuration")]
    pub battle_duration_secs: f64,

    #[serde(rename = "vehicleCompDescriptor")]
    pub tank_id: u16,
}

impl Meta {
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        serde_json::from_reader(reader).map_err(Error::JsonDecodeFailed)
    }
}
