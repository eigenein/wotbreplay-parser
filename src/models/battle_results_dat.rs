use std::io::Read;

use prost::Message;
use serde::Deserialize;
use serde_bytes::ByteBuf;

use crate::error::Error;
use crate::models::BattleResults;
use crate::result::Result;

/// Represents un-pickled `battle_results.dat`.
#[derive(Debug, Deserialize)]
#[serde(from = "(u64, ByteBuf)")]
pub struct BattleResultsDat {
    pub arena_unique_id: u64,

    /// Protobuf-serialized battle results.
    pub buffer: ByteBuf,
}

impl From<(u64, ByteBuf)> for BattleResultsDat {
    fn from((arena_unique_id, buffer): (u64, ByteBuf)) -> Self {
        Self {
            arena_unique_id,
            buffer,
        }
    }
}

impl BattleResultsDat {
    /// Parses the pickled structure from the reader which contains `battle_results.dat`.
    ///
    /// It can be used either on the `battle_results.dat` for a replay archive,
    /// or directly on `**/DAVAProject/battle_results/*/*_full.dat`.
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        serde_pickle::from_reader(reader, Default::default()).map_err(Error::UnpickleFailed)
    }

    /// Decodes the battle results from the internal buffer.
    pub fn decode_battle_results(&self) -> Result<BattleResults> {
        BattleResults::decode(self.buffer.as_ref()).map_err(Error::DecodeFailed)
    }
}

impl TryInto<BattleResults> for BattleResultsDat {
    type Error = Error;

    fn try_into(self) -> Result<BattleResults> {
        self.decode_battle_results()
    }
}
