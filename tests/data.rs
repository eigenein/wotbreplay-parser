use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::prelude::*;

#[test]
fn data_ok() -> Result<()> {
    let mut replay = Replay::open(File::open("tests/replays/player_results.wotbreplay")?)?;
    let data = replay.read_data()?;

    assert_eq!(data.client_version, "9.4.0_apple");
    assert_eq!(data.author_nickname, "zeekrab");
    assert_eq!(data.arena_unique_id, 1661909200500084);
    assert_eq!(data.arena_type_id, 65567);
    assert_eq!(data.summary.battle_level, 8);
    assert_eq!(data.summary.battle_category_id, 13);
    assert!(data.summary.is_mouse_enabled);
    assert_eq!(data.summary.matchmaker_type, 2);
    assert_eq!(data.summary.camouflage_slot, 1);
    assert_eq!(data.summary.average_mmr, [-25.396859946846963, 77.58806455880404]);

    Ok(())
}
