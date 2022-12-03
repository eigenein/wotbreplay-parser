use std::io::Read;

use prost::Message;
use serde::Deserialize;
use serde_bytes::ByteBuf;

use crate::models::BattleResults;
use crate::{Error, Result};

/// Represents un-pickled `battle_results.dat`.
#[derive(Debug, Deserialize)]
pub struct BattleResultsDat(
    /// No idea what this is.
    u64,
    /// Protobuf-serialized battle results.
    ByteBuf,
);

impl BattleResultsDat {
    /// Parses the pickled structure from the reader which contains `battle_results.dat`.
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        serde_pickle::from_reader(reader, Default::default()).map_err(Error::UnpickleFailed)
    }

    /// Decodes the battle results from the internal buffer.
    pub fn decode_battle_results(&self) -> Result<BattleResults> {
        BattleResults::decode(self.1.as_ref()).map_err(Error::DecodeFailed)
    }
}
