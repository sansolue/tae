# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

`tae` is a Cargo workspace with three crates:

| Crate | Type | Purpose |
|-------|------|---------|
| `tae-core` | library | Shared archive logic: AES-256-GCM encrypt/decrypt, ZIP read/write, `FileStore` |
| `tae` | binary | Raylib game runtime — loads content, renders, handles input |
| `tae-pack` | binary | Authoring tool — packs a `data/` folder into an encrypted `data.tae` archive |

## Commands

```bash
cargo build --workspace          # compile all crates
cargo build --workspace --release
cargo run -p tae                 # run the game (looks for data/ or data.tae)
cargo run -p tae-pack -- data/ data.tae  # pack data folder into archive
cargo check --workspace          # fast type-check all crates
cargo test --workspace           # run all tests
cargo clippy --workspace         # lint
cargo fmt --all                  # format
```

## Architecture

### Workspace layout

```
tae-core/src/lib.rs     — PACK_KEY, NONCE, encrypt/decrypt, load/load_folder, FileStore, get_text
tae/src/
  main.rs               — game loop, Raylib init, wires all modules together
  archive.rs            — thin re-export of tae_core (FileStore, get_text, load)
  loader.rs             — deserialises FileStore → GameManifest, MapDef, NpcDef, DialogueDef
  world.rs              — all data types + live World state (flags, current map)
  player.rs             — grid position, try_move with wall collision
  input.rs              — keyboard → Intent enum
  trigger.rs            — evaluates Action, returns TriggerOutcome (carries then_set_flag)
  dialogue.rs           — line-by-line DialogueState (holds then_set_flag)
  renderer.rs           — Raylib draw calls: tiles, entities, player, dialogue overlay
tae-pack/src/main.rs    — walks data dir, builds ZIP in memory, encrypts, writes .tae
```

### Core game loop (each frame)

1. `input::poll` → `Intent`
2. If dialogue active: `Confirm` advances or closes it (setting `then_set_flag` on close)
3. Otherwise: `player::try_move` → on success, `world::trigger_at` finds first condition-passing trigger → `trigger::evaluate` → `TriggerOutcome`
4. `TriggerOutcome` handled in `main`: start dialogue, load new map, or nothing
5. `renderer::draw`: tiles → entities (skips inactive) → player → dialogue overlay

### Key design notes

- `world::trigger_at` checks conditions against `world.flags` and returns the first matching trigger at a tile — order in the TOML file is the priority order.
- `world::entity_active` gates both rendering and (implicitly) trigger activation for entity-co-located triggers.
- `then_set_flag` on dialogue is stored in `DialogueState` and applied when the last line is dismissed. On `map_transition` it is applied immediately after the map loads.
- `data/` folder takes priority over `data.tae` — both are checked relative to the executable first, then the working directory (enables `cargo run`).
