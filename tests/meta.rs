#![cfg(feature = "meta")]

use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::models::map_id::MapId;
use wotbreplay_parser::replay::Replay;

#[test]
fn meta_ok() -> Result<()> {
    let meta =
        Replay::open(File::open("replays/20221203_player_results.wotbreplay")?)?.read_meta()?;

    assert_eq!(meta.player_name, "zeekrab");
    assert_eq!(meta.arena_unique_id, "1661909200500084");
    assert_eq!(meta.battle_duration_secs, 164.08485);
    assert_eq!(meta.tank_id, 26657);
    assert_eq!(meta.map_id, MapId::DynastyPearl);

    Ok(())
}
