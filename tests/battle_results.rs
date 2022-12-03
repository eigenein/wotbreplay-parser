use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::models::TeamNumber;
use wotbreplay_parser::Replay;

#[test]
fn battle_results_ok() -> Result<()> {
    let mut replay = Replay::open(File::open("tests/replays/battle_results_ok.wotbreplay")?)?;
    let battle_results = replay.read_battle_results()?;

    assert_eq!(battle_results.timestamp, 1670018359);
    assert_eq!(battle_results.players.len(), 14);

    assert_eq!(battle_results.players[0].account_id, 520886428);
    assert_eq!(battle_results.players[0].info.nickname, "77mmmr");
    assert_eq!(battle_results.players[0].info.team_number(), TeamNumber::Two);
    assert_eq!(battle_results.players[0].info.platoon_id, None);

    assert_eq!(battle_results.players[1].info.nickname, "SNAK_THE_RIPPER");
    assert_eq!(battle_results.players[1].info.team_number(), TeamNumber::One);
    assert_eq!(battle_results.players[1].info.platoon_id, Some(547466834));

    Ok(())
}
