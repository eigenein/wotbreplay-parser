use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::models::TeamNumber;
use wotbreplay_parser::Replay;

#[test]
fn battle_results_ok() -> Result<()> {
    let battle_results = Replay::open(File::open("tests/replays/battle_results_ok.wotbreplay")?)?
        .read_battle_results()?;

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

#[test]
fn player_results_ok() -> Result<()> {
    let battle_results = Replay::open(File::open("tests/replays/player_results_ok.wotbreplay")?)?
        .read_battle_results()?;

    let snak_the_ripper_info = &battle_results.player_results[3].info;
    assert_eq!(snak_the_ripper_info.base_xp, 929);
    assert_eq!(snak_the_ripper_info.damage_dealt, 2584);
    assert_eq!(snak_the_ripper_info.damage_assisted_1, 37);
    assert_eq!(snak_the_ripper_info.account_id, 566225799);
    assert_eq!(snak_the_ripper_info.display_rating(), Some(4070));
    assert_eq!(snak_the_ripper_info.n_shots, 6);
    assert_eq!(snak_the_ripper_info.n_hits_dealt, 5);
    assert_eq!(snak_the_ripper_info.n_penetrations_dealt, 5);
    assert_eq!(snak_the_ripper_info.tank_id, 5233);
    assert_eq!(snak_the_ripper_info.n_enemies_damaged, 3);
    assert_eq!(snak_the_ripper_info.n_enemies_destroyed, 2);

    let zeekrab_info = &battle_results.player_results[10].info;
    assert_eq!(zeekrab_info.base_xp, 701);
    assert_eq!(zeekrab_info.damage_dealt, 1438);
    assert_eq!(zeekrab_info.damage_assisted_2, 31);
    assert_eq!(zeekrab_info.damage_blocked, 190);
    assert_eq!(zeekrab_info.account_id, 594778041);
    assert_eq!(zeekrab_info.display_rating(), Some(4277));
    assert_eq!(zeekrab_info.n_shots, 5);
    assert_eq!(zeekrab_info.n_hits_dealt, 5);
    assert_eq!(zeekrab_info.n_penetrations_dealt, 5);
    assert_eq!(zeekrab_info.n_hits_received, 3);
    assert_eq!(zeekrab_info.n_penetrations_received, 3);
    assert_eq!(zeekrab_info.n_non_penetrating_hits_received, 0);
    assert_eq!(zeekrab_info.victory_points_1, 40);
    assert_eq!(zeekrab_info.victory_points_2, 40);
    assert_eq!(zeekrab_info.tank_id, 26657);
    assert_eq!(zeekrab_info.credits, 37679);
    assert_eq!(zeekrab_info.n_enemies_damaged, 2);
    assert_eq!(zeekrab_info.n_enemies_destroyed, 1);

    let balls_soup_info = &battle_results.player_results[8].info;
    assert_eq!(balls_soup_info.damage_dealt, 2064);
    assert_eq!(balls_soup_info.display_rating(), None);
    assert_eq!(balls_soup_info.n_shots, 9);
    assert_eq!(balls_soup_info.n_hits_dealt, 8);
    assert_eq!(balls_soup_info.n_penetrations_dealt, 8);
    assert_eq!(balls_soup_info.n_hits_received, 6);
    assert_eq!(balls_soup_info.n_penetrations_received, 4);
    assert_eq!(balls_soup_info.n_non_penetrating_hits_received, 2);
    assert_eq!(balls_soup_info.tank_id, 4737);
    assert_eq!(balls_soup_info.n_enemies_damaged, 4);
    assert_eq!(balls_soup_info.n_enemies_destroyed, 1);

    let ceaser_info = &battle_results.player_results[0].info;
    assert_eq!(ceaser_info.damage_dealt, 919);
    assert_eq!(ceaser_info.display_rating(), Some(4439));
    assert_eq!(ceaser_info.n_shots, 9);
    assert_eq!(ceaser_info.n_hits_dealt, 6);
    assert_eq!(ceaser_info.n_penetrations_dealt, 5);
    assert_eq!(ceaser_info.n_enemies_damaged, 3);
    assert_eq!(ceaser_info.n_enemies_destroyed, 0);

    Ok(())
}

#[test]
fn clan_tags_ok() -> Result<()> {
    let battle_results = Replay::open(File::open("tests/replays/player_results_ok.wotbreplay")?)?
        .read_battle_results()?;

    assert_eq!(battle_results.players[1].info.clan_tag.as_deref(), Some("AN0NY"));
    assert_eq!(battle_results.players[10].info.clan_tag.as_deref(), Some("BBS"));
    assert_eq!(battle_results.players[11].info.clan_tag, None);

    Ok(())
}
