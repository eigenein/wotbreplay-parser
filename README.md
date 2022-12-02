# `wotbreplay-parser`

World of Tanks Blitz replay parser in Rust.

[![Last commit](https://img.shields.io/github/last-commit/eigenein/wotbreplay-parser)](https://github.com/eigenein/wotbreplay-parser/commits/main)

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
