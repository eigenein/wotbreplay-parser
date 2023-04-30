use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::models::data::payload::Payload;
use wotbreplay_parser::replay::Replay;

#[test]
fn data_ok() -> Result<()> {
    let mut replay = Replay::open(File::open(
        "tests/replays/20230429_0126__helaas_pindakaas_A140_ASTRON_REX_105_16114615898101270.wotbreplay",
    )?)?;
    let data = replay.read_data()?;

    assert_eq!(data.client_version, "9.8.5_apple");
    assert_eq!(data.packets.len(), 60139);

    let (author_nickname, arena_unique_id, arena_type_id, type_0) = data
        .packets
        .iter()
        .filter_map(|packet| {
            if let Payload::Type0 {
                author_nickname,
                arena_unique_id,
                arena_type_id,
                type_0,
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
    assert_eq!(type_0.battle_level, 9);
    assert_eq!(type_0.battle_category_id, 13);
    assert!(type_0.is_mouse_enabled);
    assert_eq!(type_0.matchmaker_type, 2);
    assert_eq!(type_0.camouflage_slot, 1);
    assert_eq!(type_0.average_mmr, [Some(59.29457753400008), Some(99.34182121604681)]);
    assert_eq!(
        type_0.account_database_ids,
        [
            517033644, 537867068, 597583401, 601893865, 576810156, 531053956, 599580824, 558025185,
            526927603, 594778041, 505587490, 597422303, 531923111, 587617355
        ]
    );

    Ok(())
}
