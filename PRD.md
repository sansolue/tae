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

[[entities]]
id  = "elder"
x   = 5
y   = 3

[[triggers]]
x      = 10
y      = 0
action = { type = "map_transition", target_map = "dungeon", target_x = 1, target_y = 13 }

[[triggers]]
x      = 5
y      = 3
action = { type = "dialogue", id = "elder_intro" }
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

```
main.rs
├── archive.rs      — decrypt + unpack data.tae into memory
├── loader.rs       — deserialize TOML/JSON/YAML into engine structs
├── world.rs        — Map, Entity, Trigger types; world state
├── player.rs       — position, movement, collision
├── trigger.rs      — trigger evaluation and action dispatch
├── dialogue.rs     — dialogue state machine (active line, speaker, advance)
├── renderer.rs     — Raylib draw calls (tiles, entities, dialogue box)
└── input.rs        — keyboard → intent mapping (Move, Interact, Confirm)
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

## Trigger Actions (MVP)

| Action type       | Effect                                          |
|-------------------|-------------------------------------------------|
| `dialogue`        | Opens dialogue overlay; blocks movement         |
| `map_transition`  | Loads target map, places player at target coord |
| `set_flag`        | Sets a named boolean flag in world state        |
| `conditional`     | Fires a sub-action only if a named flag is set  |

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

## Encryption

- Algorithm: **AES-256-GCM**.
- Key derivation: hardcoded key constant in the binary for MVP (rotate to env-var or key file post-MVP).
- A separate `tae-pack` CLI subcommand (or script) encrypts a plain folder into `data.tae` at authoring time.

---

## MVP Acceptance Criteria

1. Binary starts, finds and decrypts `data/data.tae`, opens a Raylib window.
2. Player can move across a tile map using arrow keys with wall collision.
3. Walking onto a trigger tile fires the configured action (dialogue or map transition).
4. Dialogue overlay renders, advances line by line, then closes — returning control to the player.
5. `map_transition` correctly unloads the current map and loads the target, placing the player at the specified coordinates.
6. `set_flag` / `conditional` triggers work so a dialogue or transition can be gated on a prior event.
7. A sample game with two maps and three NPCs ships alongside the engine as the reference content.