use std::io::Read;

use serde::Deserialize;
use serde_bytes::ByteBuf;

use crate::{Error, Result};

/// Represents un-pickled `battle_results.dat`.
#[derive(Debug, Deserialize)]
pub struct BattleResultsDat(pub(crate) u64, pub(crate) ByteBuf);

impl BattleResultsDat {
    /// Try and parse the pickled structure from the reader which contains `battle_results.dat`.
    pub fn from_reader(reader: impl Read) -> Result<Self> {
        serde_pickle::from_reader(reader, Default::default()).map_err(Error::UnpickleFailed)
    }
}
