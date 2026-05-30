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

## Building

```sh
cargo build --release
```

Requires a C compiler and cmake for the Raylib build step (handled automatically by the `raylib` crate).

## Running

```sh
cargo run
```

The engine looks for game content in this order (relative to the executable):
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

[[entities]]
id        = "elder"
x         = 5
y         = 3
condition = { flag = "elder_left", present = false }  # optional

[[triggers]]
x         = 5
y         = 3
condition = { flag = "talked_to_elder", present = false }  # optional; first match wins
action    = { type = "dialogue", id = "elder_intro", then_set_flag = "talked_to_elder" }
```

### Trigger actions

| Type             | Fields                                                    |
|------------------|-----------------------------------------------------------|
| `dialogue`       | `id` — dialogue id to open                               |
| `map_transition` | `target_map`, `target_x`, `target_y`                     |
| `set_flag`       | `flag` — flag name to set                                |

All action types accept an optional `then_set_flag` field, applied after the action fully completes.

### Conditions

```toml
condition = { flag = "some_flag", present = true }  # fires when flag IS set
condition = { flag = "some_flag" }                  # fires when flag is NOT set (default)
```

Conditions can be placed on both triggers and entity placements.

### Controls

| Key              | Action        |
|------------------|---------------|
| Arrow keys / WASD | Move         |
| Space / Enter    | Advance dialogue |

## License

MIT
