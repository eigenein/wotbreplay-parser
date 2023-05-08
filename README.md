# `wotbreplay-parser`

World of Tanks Blitz replay parser in Rust.

[![Crates.io](https://img.shields.io/crates/v/wotbreplay-parser)](https://crates.io/crates/wotbreplay-parser)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/wotbreplay-parser)](https://github.com/eigenein/wotbreplay-parser/commits/main)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/eigenein/wotbreplay-parser/check.yaml)](https://github.com/eigenein/wotbreplay-parser/actions)
![License: MIT](https://img.shields.io/crates/l/wotbreplay-parser)
[![docs.rs](https://img.shields.io/docsrs/wotbreplay-parser)](https://docs.rs/wotbreplay-parser)

## Quickstart

```rust
use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::models::battle_results::TeamNumber;
use wotbreplay_parser::replay::Replay;

fn main() -> Result<()> {
    let battle_results = Replay::open(File::open("replays/20221203_player_results.wotbreplay")?)?.read_battle_results()?;

    assert_eq!(battle_results.timestamp, 1670083956);
    assert_eq!(battle_results.players.len(), 14);

    assert_eq!(battle_results.players[0].account_id, 595693744);
    assert_eq!(battle_results.players[0].info.nickname, "yuranhik_hustriy26");
    assert_eq!(battle_results.players[0].info.team(), TeamNumber::One);
    assert_eq!(battle_results.players[0].info.platoon_id, Some(545104609));

    assert_eq!(battle_results.players[1].info.nickname, "SNAK_THE_RIPPER");
    assert_eq!(battle_results.players[1].info.team(), TeamNumber::Two);
    assert_eq!(battle_results.players[1].info.platoon_id, Some(273692628));

    Ok(())
}
```
