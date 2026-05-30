# tae — MVP Product Requirements Document

## Overview

`tae` (Text Adventure Engine) is a standalone Raylib-backed game runtime for authoring and playing map-and-trigger driven RPG-style interactive stories. Authors define their game entirely in data files; players run the `tae` binary and the engine handles rendering, input, and game logic.

---

## Goals

- Playable, self-contained RPG-style experience from a single binary + data archive.
- Zero-code authoring: game content is expressed in TOML/YAML/JSON, not Rust.
- Encrypted archive format (`.tae`) keeps shipped assets opaque.
- Clear separation between engine (Rust) and game content (data files).

## Non-Goals (post-MVP)

- Audio / music system.
- Combat or inventory mechanics.
- Multiple save slots.
- A visual map editor.
- Scripting language / Lua integration.

---

## Data Loading

The runtime resolves content using the following priority order (relative to the executable):

1. **`data/` folder** — if present, files are read directly from disk. Intended for development and authoring.
2. **`data.tae` file** — if no `data/` folder is found, the engine loads this AES-encrypted ZIP into memory and reads files from it. Intended for distribution.

```
# Development layout (folder takes priority)
data/
├── game.toml
├── maps/
│   ├── town.toml
│   └── dungeon.toml
├── entities/
│   └── npcs.toml
└── dialogue/
    └── town_npcs.toml

# Distribution layout (folder absent, archive used)
data.tae          ← encrypted zip with the same internal structure
```

`data.tae` is an **AES-256-GCM encrypted ZIP**. On load the engine decrypts it entirely in memory — nothing is written to disk.

---

## Data File Schema (MVP)

### `game.toml` — manifest
```toml
title        = "My Adventure"
start_map    = "town"
tile_size    = 32          # pixels
window_w     = 800
window_h     = 600
```

### `maps/<id>.toml` — map definition
```toml
id     = "town"
width  = 20       # tiles
height = 15

# Row-major tile grid; integers reference a future tileset (use 0/1 for MVP: floor/wall)
tiles  = [
  [1,1,1,...],
  ...
]

# Entity hidden once the flag is set (condition absent by default)
[[entities]]
id        = "elder"
x         = 5
y         = 3
condition = { flag = "elder_left" }

[[triggers]]
x      = 10
y      = 0
action = { type = "map_transition", target_map = "dungeon", target_x = 1, target_y = 13 }

# First-time dialogue; sets a flag when it ends
[[triggers]]
x        = 5
y        = 3
condition = { flag = "talked_to_elder", absent = true }
action   = { type = "dialogue", id = "elder_intro", then_set_flag = "talked_to_elder" }

# Follow-up dialogue shown only after the first conversation
[[triggers]]
x        = 5
y        = 3
condition = { flag = "talked_to_elder", present = true }
action   = { type = "dialogue", id = "elder_followup" }
```

### `entities/npcs.toml`
```toml
[[npc]]
id    = "elder"
name  = "Village Elder"
glyph = "E"           # single char rendered in MVP; sprite in post-MVP
```

### `dialogue/<id>.toml`
```toml
[[dialogue]]
id = "elder_intro"

[[dialogue.lines]]
speaker = "Village Elder"
text    = "Welcome, traveller."

[[dialogue.lines]]
speaker = "Village Elder"
text    = "The dungeon to the north holds great danger."
```

---

## Engine Architecture

The project is a Cargo workspace with three crates:

| Crate | Type | Purpose |
|-------|------|---------|
| `tae-core` | library | AES-256-GCM encrypt/decrypt, ZIP read, `FileStore`, `get_text` |
| `tae` | binary | Raylib game runtime |
| `tae-pack` | binary | Authoring tool — packs `data/` into `data.tae` |

### `tae` module layout

```
tae/src/
  main.rs       — game loop, Raylib init, wires all modules
  archive.rs    — re-exports tae_core (FileStore, get_text, load)
  loader.rs     — deserialises FileStore → GameManifest, MapDef, NpcDef, DialogueDef
  world.rs      — all data types + live World state (flags, current map)
  player.rs     — grid position, try_move with wall collision
  input.rs      — keyboard → Intent enum
  trigger.rs    — evaluates Action, returns TriggerOutcome (carries then_set_flag)
  dialogue.rs   — line-by-line DialogueState (holds then_set_flag)
  renderer.rs   — Raylib draw calls: tiles, entities, player, dialogue overlay
```

### Core Loop

```
each frame:
  1. input.rs     → Intent
  2. player.rs    → attempt move (collision check against world)
  3. trigger.rs   → check player tile for triggers; fire if matched
  4. world.rs     → apply any state mutations (map transition, flag set)
  5. renderer.rs  → draw tiles → entities → player → UI overlay
```

---

## Conditions

A `condition` block can be attached to any **trigger** or **entity placement**. If the condition is not met, the trigger is skipped or the entity is hidden and non-collidable.

```toml
condition = { flag = "some_flag", present = true }   # fires only when flag IS set
condition = { flag = "some_flag" }                   # fires only when flag is NOT set (default)
```

Multiple triggers at the same tile are evaluated in definition order — the first whose condition passes wins.

---

## Trigger Actions

| Action type       | Effect                                          |
|-------------------|-------------------------------------------------|
| `dialogue`        | Opens dialogue overlay; blocks movement         |
| `map_transition`  | Loads target map, places player at target coord |
| `set_flag`        | Sets a named boolean flag in world state        |

All action types accept an optional `then_set_flag` field. The flag is set **after** the action fully completes (e.g., after the last dialogue line is dismissed, after the map transition fires).

```toml
action = { type = "dialogue", id = "elder_intro", then_set_flag = "talked_to_elder" }
action = { type = "map_transition", target_map = "dungeon", target_x = 1, target_y = 1, then_set_flag = "entered_dungeon" }
```

The old `conditional` action type is replaced by the `condition` field on triggers — cleaner and more composable.

---

## Player & Movement

- Grid-based movement (one tile per keypress): WASD or arrow keys.
- Tile `1` = wall (impassable), tile `0` = floor.
- On move into a trigger tile: trigger fires before the move resolves (player stays on origin tile for `dialogue`; teleports for `map_transition`).

---

## Rendering (Raylib, MVP)

- Tiles: coloured rectangles (no sprite sheet yet). Wall = dark grey, floor = light grey.
- Entities: single character centred on their tile (Raylib `DrawText`).
- Player: white rectangle with `@` glyph.
- Dialogue box: semi-transparent panel at bottom 25% of window; speaker name + current line; `[Space/Enter] to advance`.
- No camera scroll in MVP — map must fit within window tile grid.

---

## Encryption & Packing

- Algorithm: **AES-256-GCM** with a fixed nonce (MVP). Rotate per-archive post-MVP.
- Key: hardcoded constant in `tae-core` — single source of truth shared by both `tae` (decrypt) and `tae-pack` (encrypt).
- `tae-pack` is a **separate binary** (not a subcommand) so it is never shipped alongside the game runtime.

```sh
tae-pack <data_dir> [output.tae]
# e.g. tae-pack data/ data.tae
```

---

## MVP Acceptance Criteria

1. Binary starts, finds `data/` folder or `data.tae` archive, opens a Raylib window.
   `tae-pack` can pack a `data/` folder into a `data.tae` that the runtime loads correctly.
2. Player can move across a tile map using arrow keys with wall collision.
3. Walking onto a trigger tile fires the configured action (dialogue or map transition).
4. Dialogue overlay renders, advances line by line, then closes — returning control to the player.
5. `map_transition` correctly unloads the current map and loads the target, placing the player at the specified coordinates.
6. Trigger `condition` gates correctly — first matching trigger at a tile fires, others are skipped.
7. Entity `condition` hides the entity (no render, no collision) when the condition is not met.
8. `then_set_flag` on any action sets the flag after the action fully completes.
9. A sample game with two maps and NPCs ships alongside the engine as the reference content, exercising conditions and `then_set_flag`.