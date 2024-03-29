use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::models::data::entity_method::EntityMethod;
use wotbreplay_parser::models::data::payload::Payload;
use wotbreplay_parser::replay::Replay;

#[test]
fn parse_ok() -> Result<()> {
    let mut replay =
        Replay::open(File::open("replays/20230429_0126__helaas_pindakaas.wotbreplay")?)?;
    let data = replay.read_data()?;

    assert_eq!(data.client_version, "9.8.5_apple");
    assert_eq!(data.packets.len(), 60139);

    Ok(())
}

#[test]
fn base_player_create_ok() -> Result<()> {
    let mut replay =
        Replay::open(File::open("replays/20230429_0126__helaas_pindakaas.wotbreplay")?)?;
    let data = replay.read_data()?;

    let (author_nickname, arena_unique_id, arena_type_id, arguments) = data
        .packets
        .iter()
        .filter_map(|packet| {
            if let Payload::BasePlayerCreate {
                author_nickname,
                arena_unique_id,
                arena_type_id,
                arguments: type_0,
            } = &packet.payload
            {
                Some((author_nickname, arena_unique_id, arena_type_id, type_0))
            } else {
                None
            }
        })
        .next()
        .unwrap();
    assert_eq!(author_nickname, "helaas_pindakaas");
    assert_eq!(*arena_unique_id, 16114615898101270);
    assert_eq!(*arena_type_id, 43);
    assert_eq!(arguments.battle_level, 9);
    assert_eq!(arguments.battle_category_id, 13);
    assert!(arguments.is_mouse_enabled);
    assert_eq!(arguments.matchmaker_type, 2);
    assert_eq!(arguments.camouflage_slot, 1);
    assert_eq!(
        arguments.average_mm_ratings,
        Some(vec![Some(59.29457753400008), Some(99.34182121604681)])
    );
    assert_eq!(
        arguments.account_database_ids,
        Some(vec![
            517033644, 537867068, 597583401, 601893865, 576810156, 531053956, 599580824, 558025185,
            526927603, 594778041, 505587490, 597422303, 531923111, 587617355
        ])
    );

    Ok(())
}

#[test]
fn update_arena_ok() -> Result<()> {
    let mut replay =
        Replay::open(File::open("replays/20230429_0126__helaas_pindakaas.wotbreplay")?)?;
    let data = replay.read_data()?;

    let players = data
        .packets
        .iter()
        .filter_map(|packet| match &packet.payload {
            Payload::EntityMethod(EntityMethod::UpdateArena { arguments, .. }) => {
                arguments.players.as_ref()
            }
            _ => None,
        })
        .next()
        .unwrap();

    assert_eq!(players.players.len(), 14);
    assert_eq!(players.players[1].team_number, 1);
    assert_eq!(players.players[1].nickname, "Igrok_WoT_blitz");
    assert_eq!(players.players[1].account_id, 597583401);
    assert_eq!(players.players[1].platoon_number, None);
    assert_eq!(players.players[1].clan_tag, Some("F_ANG".to_string()));

    Ok(())
}

#[test]
fn training_room_ok() -> Result<()> {
    let mut replay = Replay::open(File::open("replays/20230503_training_room.wotbreplay")?)?;
    replay.read_data()?;
    Ok(())
}
