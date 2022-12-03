use serde::Deserialize;
use serde_bytes::ByteBuf;

/// Represents un-pickled `battle_results.dat`.
#[derive(Debug, Deserialize)]
pub struct BattleResultsDat(pub(crate) u64, pub(crate) ByteBuf);
