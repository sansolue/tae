# tae — Text Adventure Engine

A Raylib-backed runtime for building map-and-trigger driven RPG-style interactive stories. Authors define their game entirely in TOML data files; players run the `tae` binary.

## Features

- Grid-based tile maps with wall collision
- NPC entities with glyph rendering
- Trigger zones — dialogue, map transitions, flag setting
- Condition system — show/hide entities and gate triggers based on world flags
- `then_set_flag` on any action — set a flag after an action completes
- Encrypted `.tae` archive format for distribution (AES-256-GCM)
- Dev mode: reads from a plain `data/` folder for fast iteration
- `tae-pack` authoring tool — packs a `data/` folder into a `data.tae` archive

## Workspace

| Crate | Role |
|-------|------|
| `tae-core` | Shared library — encryption, archive loading |
| `tae` | Game runtime binary |
| `tae-pack` | Packing tool binary |

## Building

```sh
cargo build --workspace --release
```

Requires a C compiler and cmake for the Raylib build step (handled automatically by the `raylib` crate).

## Running

```sh
# Development — reads from data/ folder directly
cargo run -p tae

# Pack data folder into an encrypted archive
cargo run -p tae-pack -- data/ data.tae

# Distribution — ship the tae binary alongside data.tae
./tae
```

The engine resolves content in this order (relative to the executable, then working directory):
1. `data/` folder — used during development
2. `data.tae` — encrypted archive used for distribution

A sample two-map game is included in `data/`.

## Game Content Structure

```
data/
├── game.toml              # manifest: title, start map, window size, tile size
├── maps/
│   └── <id>.toml          # tile grid, entity placements, triggers
├── entities/
│   └── npcs.toml          # NPC definitions (id, name, glyph)
└── dialogue/
    └── <name>.toml        # dialogue trees
```

### `game.toml`

```toml
title     = "My Game"
start_map = "town"
tile_size = 32
window_w  = 640
window_h  = 480
```

### Map file

```toml
id     = "town"
width  = 20
height = 15

# 0 = floor, 1 = wall (row-major)
tiles = [
  [1,1,1,...],
  ...
]

# Entity hidden once the flag is set
[[entities]]
id        = "elder"
x         = 5
y         = 3
condition = { flag = "elder_left" }   # absent by default; omit condition to always show

# First visit — sets flag on dismiss
[[triggers]]
x         = 5
y         = 3
condition = { flag = "talked_to_elder" }   # absent by default
action    = { type = "dialogue", id = "elder_intro", then_set_flag = "talked_to_elder" }

# Follow-up on subsequent visits
[[triggers]]
x         = 5
y         = 3
condition = { flag = "talked_to_elder", present = true }
action    = { type = "dialogue", id = "elder_followup" }
```

### Trigger actions

| Type             | Fields                                                |
|------------------|-------------------------------------------------------|
| `dialogue`       | `id` — dialogue id to open                           |
| `map_transition` | `target_map`, `target_x`, `target_y`                 |
| `set_flag`       | `flag` — flag name to set immediately                |

All action types accept an optional `then_set_flag` field, applied after the action fully completes.

### Conditions

```toml
condition = { flag = "some_flag", present = true }  # requires flag IS set
condition = { flag = "some_flag" }                  # requires flag is NOT set (default)
```

Multiple triggers at the same tile fire in definition order — first matching condition wins.
Conditions can be placed on both triggers and entity placements.

### Controls

| Key               | Action              |
|-------------------|---------------------|
| Arrow keys / WASD | Move                |
| Space / Enter     | Advance dialogue    |

## License

MIT
