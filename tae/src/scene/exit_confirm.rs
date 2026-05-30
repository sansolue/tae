use raylib::prelude::*;

use super::{EngineState, Scene, SceneTransition};

pub struct ExitConfirmScene {
    cursor: usize, // 0 = No, 1 = Yes
}

impl ExitConfirmScene {
    pub fn new() -> Self {
        Self { cursor: 0 }
    }
}

impl Scene for ExitConfirmScene {
    fn is_overlay(&self) -> bool { true }

    fn update(&mut self, rl: &mut RaylibHandle, _engine: &mut EngineState) -> SceneTransition {
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_RIGHT) {
            self.cursor = 1 - self.cursor;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return SceneTransition::Pop;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return if self.cursor == 1 {
                SceneTransition::Quit
            } else {
                SceneTransition::Pop
            };
        }
        SceneTransition::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState) {
        let w = engine.manifest.window_w as i32;
        let h = engine.manifest.window_h as i32;
        // Semi-transparent overlay
        d.draw_rectangle(0, 0, w, h, Color { r: 0, g: 0, b: 0, a: 160 });

        let box_w = 300;
        let box_h = 120;
        let bx = (w - box_w) / 2;
        let by = (h - box_h) / 2;
        d.draw_rectangle(bx, by, box_w, box_h, Color { r: 20, g: 20, b: 20, a: 240 });
        d.draw_rectangle_lines(bx, by, box_w, box_h, Color::GRAY);

        d.draw_text("Exit game?", bx + 100, by + 20, 20, Color::WHITE);

        let no_col = if self.cursor == 0 { Color::YELLOW } else { Color::GRAY };
        let yes_col = if self.cursor == 1 { Color::YELLOW } else { Color::GRAY };
        d.draw_text("No", bx + 70, by + 72, 18, no_col);
        d.draw_text("Yes", bx + 180, by + 72, 18, yes_col);
        d.draw_text("← / →  to choose,  Enter to confirm", bx + 12, by + 100, 10, Color::DARKGRAY);
    }
}
