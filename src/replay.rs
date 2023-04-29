use std::io::{Read, Seek};

use zip::ZipArchive;

#[cfg(feature = "meta")]
use crate::models::Meta;
use crate::models::{BattleResults, BattleResultsDat, Data};
use crate::result::Result;

/// Opened (but not yet parsed) replay.
pub struct Replay<R>(pub(crate) ZipArchive<R>);

impl<R: Read + Seek> Replay<R> {
    /// Opens a replay from the reader which contains a `*.wotbreplay`.
    pub fn open(reader: R) -> Result<Self> {
        Ok(Self(ZipArchive::new(reader)?))
    }

    /// Reads and parses the battle results from the replay.
    pub fn read_battle_results(&mut self) -> Result<BattleResults> {
        self.read_battle_results_dat()?.try_into()
    }

    /// Reads and parses the included `battle_results.dat`.
    pub fn read_battle_results_dat(&mut self) -> Result<BattleResultsDat> {
        let pickled_battle_results = self.0.by_name("battle_results.dat")?;
        BattleResultsDat::from_reader(pickled_battle_results)
    }

    #[cfg(feature = "meta")]
    pub fn read_meta(&mut self) -> Result<Meta> {
        Meta::from_reader(self.0.by_name("meta.json")?)
    }

    pub fn read_data(&mut self) -> Result<Data> {
        Data::from_reader(self.0.by_name("data.wotreplay")?)
    }
}
