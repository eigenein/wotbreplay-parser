use std::io::{Read, Seek};

use prost::Message;
use zip::ZipArchive;

use crate::error::Error;
use crate::models::{BattleResults, BattleResultsDat};
use crate::result::Result;

pub struct Replay<R>(pub(crate) ZipArchive<R>);

impl<R: Read + Seek> Replay<R> {
    /// Opens a replay from the reader.
    pub fn open(reader: R) -> Result<Self> {
        ZipArchive::new(reader)
            .map_err(Error::OpenArchiveFailed)
            .map(Self)
    }

    pub fn read_battle_results(&mut self) -> Result<BattleResults> {
        let mut pickled_battle_results = self
            .0
            .by_name("battle_results.dat")
            .map_err(Error::OpenBattleResultsFailed)?;
        let mut serialized_battle_results = Vec::new();
        pickled_battle_results
            .read_to_end(&mut serialized_battle_results)
            .map_err(Error::ReadEntryFailed)?;
        let battle_results_dat: BattleResultsDat =
            serde_pickle::from_slice(&serialized_battle_results, Default::default())
                .map_err(Error::UnpickleFailed)?;
        BattleResults::decode(battle_results_dat.1.as_ref()).map_err(Error::DecodeFailed)
    }
}
