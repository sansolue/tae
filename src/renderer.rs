use raylib::prelude::*;

use crate::dialogue::DialogueState;
use crate::player::Player;
use crate::world::World;

const COLOR_FLOOR: Color = Color { r: 50, g: 50, b: 50, a: 255 };
const COLOR_WALL: Color = Color { r: 20, g: 20, b: 20, a: 255 };
const COLOR_PLAYER: Color = Color::WHITE;
const COLOR_NPC: Color = Color { r: 100, g: 200, b: 255, a: 255 };
const COLOR_DIALOGUE_BG: Color = Color { r: 0, g: 0, b: 0, a: 200 };
const COLOR_DIALOGUE_TEXT: Color = Color::WHITE;
const COLOR_SPEAKER: Color = Color { r: 255, g: 220, b: 80, a: 255 };

pub fn draw(
    d: &mut RaylibDrawHandle,
    world: &World,
    player: &Player,
    dialogue: Option<&DialogueState>,
) {
    let ts = world.manifest.tile_size as i32;
    let ww = world.manifest.window_w as i32;
    let wh = world.manifest.window_h as i32;

    d.clear_background(Color::BLACK);

    // Tiles
    for (row_idx, row) in world.current_map.tiles.iter().enumerate() {
        for (col_idx, &tile) in row.iter().enumerate() {
            let rx = col_idx as i32 * ts;
            let ry = row_idx as i32 * ts;
            let color = if tile == 1 { COLOR_WALL } else { COLOR_FLOOR };
            d.draw_rectangle(rx, ry, ts, ts, color);
        }
    }

    // Entities (NPCs)
    for placement in &world.current_map.entities {
        if let Some(npc) = world.npcs.get(&placement.id) {
            let rx = placement.x as i32 * ts;
            let ry = placement.y as i32 * ts;
            d.draw_rectangle(rx, ry, ts, ts, COLOR_NPC);
            let glyph = npc.glyph.to_string();
            d.draw_text(&glyph, rx + ts / 4, ry + ts / 4, ts / 2, Color::BLACK);
        }
    }

    // Player
    {
        let rx = player.x as i32 * ts;
        let ry = player.y as i32 * ts;
        d.draw_rectangle(rx, ry, ts, ts, COLOR_PLAYER);
        d.draw_text("@", rx + ts / 4, ry + ts / 4, ts / 2, Color::BLACK);
    }

    // Dialogue overlay
    if let Some(state) = dialogue {
        if let Some((speaker, text)) = state.current() {
            let box_h = wh / 4;
            let box_y = wh - box_h;
            d.draw_rectangle(0, box_y, ww, box_h, COLOR_DIALOGUE_BG);
            d.draw_rectangle_lines(0, box_y, ww, box_h, Color::GRAY);

            d.draw_text(speaker, 16, box_y + 10, 18, COLOR_SPEAKER);
            d.draw_text(text, 16, box_y + 34, 16, COLOR_DIALOGUE_TEXT);
            d.draw_text(
                "[Space / Enter] to continue",
                16,
                box_y + box_h - 22,
                12,
                Color::GRAY,
            );
        }
    }
}
