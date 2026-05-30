use raylib::prelude::*;

use super::{EngineState, Scene, SceneTransition};

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Save,
    Load,
}

pub struct SaveLoadScene {
    mode: Mode,
    page: usize,
    cursor: usize,
}

impl SaveLoadScene {
    pub fn save() -> Self {
        Self { mode: Mode::Save, page: 0, cursor: 0 }
    }

    pub fn load() -> Self {
        Self { mode: Mode::Load, page: 0, cursor: 0 }
    }
}

impl Scene for SaveLoadScene {
    fn update(&mut self, rl: &mut RaylibHandle, engine: &mut EngineState) -> SceneTransition {
        let slots_per_page = engine.saves.slots_per_page;
        let page_count = engine.saves.page_count();
        let slots_on_page = engine.saves.page_slots(self.page).len();

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return SceneTransition::Pop;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
            if self.cursor > 0 { self.cursor -= 1; }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S) {
            if self.cursor + 1 < slots_on_page { self.cursor += 1; }
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) && self.page > 0 {
            self.page -= 1;
            self.cursor = 0;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && self.page + 1 < page_count {
            self.page += 1;
            self.cursor = 0;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            let slot_idx = self.page * slots_per_page + self.cursor;
            match self.mode {
                Mode::Save => {
                    let _ = engine.saves.save(slot_idx, None);
                    return SceneTransition::Pop;
                }
                Mode::Load => {
                    if engine.saves.slots[slot_idx].is_some() {
                        // GameContext restore is TBD — start fresh for now
                        return SceneTransition::Replace(Box::new(super::game::GameScene::new()));
                    }
                }
            }
        }

        SceneTransition::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState) {
        let w = engine.manifest.window_w as i32;
        let h = engine.manifest.window_h as i32;
        d.clear_background(Color { r: 10, g: 10, b: 20, a: 255 });

        let title = if self.mode == Mode::Save { "Save Game" } else { "Load Game" };
        d.draw_text(title, 40, 30, 28, Color::WHITE);
        d.draw_line(40, 65, w - 40, 65, Color::GRAY);

        let slots = engine.saves.page_slots(self.page);
        let spp = engine.saves.slots_per_page;
        let start_y = 80;

        for (i, slot) in slots.iter().enumerate() {
            let global_idx = self.page * spp + i;
            let y = start_y + i as i32 * 52;
            let is_sel = i == self.cursor;

            let bg = if is_sel {
                Color { r: 40, g: 40, b: 60, a: 255 }
            } else {
                Color { r: 20, g: 20, b: 30, a: 255 }
            };
            d.draw_rectangle(30, y, w - 60, 44, bg);
            if is_sel {
                d.draw_rectangle_lines(30, y, w - 60, 44, Color::YELLOW);
            }

            match slot {
                Some(s) => {
                    let name = s.display_name(global_idx);
                    let ts = s.display_timestamp();
                    d.draw_text(&name, 46, y + 8, 16, Color::WHITE);
                    d.draw_text(&ts, 46, y + 26, 12, Color::GRAY);
                }
                None => {
                    d.draw_text(&format!("Slot {} — Empty", global_idx + 1), 46, y + 14, 16, Color::DARKGRAY);
                }
            }
        }

        // Pagination
        let page_count = engine.saves.page_count();
        let pag = format!("< Page {} / {} >", self.page + 1, page_count);
        d.draw_text(&pag, w / 2 - 60, h - 32, 14, Color::GRAY);
        d.draw_text("Escape to return", 20, h - 18, 11, Color::DARKGRAY);
    }
}
