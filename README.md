# `wotbreplay-parser`

World of Tanks Blitz replay parser in Rust.

[![Crates.io](https://img.shields.io/crates/v/wotbreplay-parser)](https://crates.io/crates/wotbreplay-parser)
[![Last commit](https://img.shields.io/github/last-commit/eigenein/wotbreplay-parser)](https://github.com/eigenein/wotbreplay-parser/commits/main)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/eigenein/wotbreplay-parser/Check)](https://github.com/eigenein/wotbreplay-parser/actions)
![License: MIT](https://img.shields.io/crates/l/wotbreplay-parser)
[![docs.rs](https://img.shields.io/docsrs/wotbreplay-parser)](https://docs.rs/wotbreplay-parser)

## Quickstart

```rust
use std::fs::File;

use anyhow::Result;
use wotbreplay_parser::prelude::*;

fn main() -> Result<()> {
    let battle_results = Replay::open(File::open("tests/replays/battle_results.wotbreplay")?)?
        .read_battle_results()?;

    assert_eq!(battle_results.timestamp, 1670018359);
    assert_eq!(battle_results.players.len(), 14);

    assert_eq!(battle_results.players[0].account_id, 520886428);
    assert_eq!(battle_results.players[0].info.nickname, "77mmmr");
    assert_eq!(battle_results.players[0].info.team(), TeamNumber::Two);
    assert_eq!(battle_results.players[0].info.platoon_id, None);

    assert_eq!(battle_results.players[1].info.nickname, "SNAK_THE_RIPPER");
    assert_eq!(battle_results.players[1].info.team(), TeamNumber::One);
    assert_eq!(battle_results.players[1].info.platoon_id, Some(547466834));

    Ok(())
}
```

## Replay structure

`*.wotbreplay` is a ZIP-archive containing:
- `battle_results.dat`
- `meta.json`
- `data.wotreplay`

### `battle_results.dat`

This is a [pickled](https://docs.python.org/3/library/pickle.html) 2-tuple:
- Some big number
- Battle results serialized with [Protocol Buffers](https://developers.google.com/protocol-buffers)

#### Useful tools

- [Protobuf Decoder](https://protobuf-decoder.netlify.app/)
- [`protobuf-inspector`](https://github.com/mildsunrise/protobuf-inspector)
- [Hex Fiend](https://hexfiend.com/)

### `meta.json`

### `data.wotreplay`
