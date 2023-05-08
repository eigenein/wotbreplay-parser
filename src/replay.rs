use std::io::{Read, Seek};

use zip::ZipArchive;

use crate::models::battle_results::BattleResults;
use crate::models::battle_results_dat::BattleResultsDat;
use crate::models::data::Data;
#[cfg(feature = "meta")]
use crate::models::meta::Meta;
use crate::result::Result;

/// World of Tanks Blitz replay.
///
/// # Replay structure
///
/// `*.wotbreplay` is a ZIP-archive containing:
///
/// - `battle_results.dat`
/// - `meta.json`
/// - `data.wotreplay`
pub struct Replay<R>(pub(crate) ZipArchive<R>);

impl<R: Read + Seek> Replay<R> {
    /// Open a replay from the reader which contains a `*.wotbreplay`.
    pub fn open(reader: R) -> Result<Self> {
        Ok(Self(ZipArchive::new(reader)?))
    }

    /// Read and parse the full battle results structure from `battle_results.dat`.
    pub fn read_battle_results(&mut self) -> Result<BattleResults> {
        self.read_battle_results_dat()?.try_into()
    }

    /// Read and parse the root 2-tuple from `battle_results.dat`.
    pub fn read_battle_results_dat(&mut self) -> Result<BattleResultsDat> {
        let pickled_battle_results = self.0.by_name("battle_results.dat")?;
        BattleResultsDat::from_reader(pickled_battle_results)
    }

    /// Read and parse the included `meta.json`.
    #[cfg(feature = "meta")]
    pub fn read_meta(&mut self) -> Result<Meta> {
        Meta::from_reader(self.0.by_name("meta.json")?)
    }

    /// Read and parse the included `data.wotreplay`.
    pub fn read_data(&mut self) -> Result<Data> {
        Data::from_reader(self.0.by_name("data.wotreplay")?)
    }
}
