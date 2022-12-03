use std::io::{Read, Seek};

use prost::Message;
use zip::ZipArchive;

use crate::error::Error;
use crate::models::{BattleResults, BattleResultsDat};
use crate::result::Result;

/// Opened (but not yet parsed) replay.
pub struct Replay<R>(pub(crate) ZipArchive<R>);

impl<R: Read + Seek> Replay<R> {
    /// Opens a replay from the reader which contains a `*.wotbreplay`.
    pub fn open(reader: R) -> Result<Self> {
        ZipArchive::new(reader)
            .map_err(Error::OpenArchiveFailed)
            .map(Self)
    }

    /// Reads and parses the battle results from the replay.
    pub fn read_battle_results(&mut self) -> Result<BattleResults> {
        let pickled_battle_results = self
            .0
            .by_name("battle_results.dat")
            .map_err(Error::OpenBattleResultsFailed)?;
        let battle_results_dat = BattleResultsDat::from_reader(pickled_battle_results)?;
        BattleResults::decode(battle_results_dat.1.as_ref()).map_err(Error::DecodeFailed)
    }
}
