use raylib::prelude::*;

use super::{EngineState, Scene, SceneTransition};
use crate::asset;

const ITEMS: &[&str] = &["New Game", "Load", "Settings", "About", "Exit"];

pub struct MainMenuScene {
    texture: Option<Texture2D>,
    cursor: usize,
    loaded: bool,
}

impl MainMenuScene {
    pub fn new() -> Self {
        Self { texture: None, cursor: 0, loaded: false }
    }
}

impl Scene for MainMenuScene {
    fn on_load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, engine: &EngineState) {
        if self.loaded { return; }
        if let Some(path) = &engine.manifest.main_menu.background.clone() {
            self.texture = asset::load_texture(rl, thread, &engine.store, &path);
        }
        self.loaded = true;
    }

    fn update(&mut self, rl: &mut RaylibHandle, _engine: &mut EngineState) -> SceneTransition {
        if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
            self.cursor = self.cursor.saturating_sub(1);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S) {
            self.cursor = (self.cursor + 1).min(ITEMS.len() - 1);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return SceneTransition::Push(Box::new(super::exit_confirm::ExitConfirmScene::new()));
        }
        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            return match self.cursor {
                0 => SceneTransition::Push(Box::new(super::game::GameScene::new())),
                1 => SceneTransition::Push(Box::new(super::save_load::SaveLoadScene::load())),
                2 => SceneTransition::Push(Box::new(super::settings::SettingsScene::new())),
                3 => SceneTransition::Push(Box::new(super::about::AboutScene::new())),
                4 => SceneTransition::Push(Box::new(super::exit_confirm::ExitConfirmScene::new())),
                _ => SceneTransition::None,
            };
        }
        SceneTransition::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState) {
        let w = engine.manifest.window_w as i32;
        let h = engine.manifest.window_h as i32;

        d.clear_background(Color { r: 10, g: 10, b: 20, a: 255 });
        if let Some(tex) = &self.texture {
            let src = Rectangle { x: 0.0, y: 0.0, width: tex.width as f32, height: tex.height as f32 };
            let dst = Rectangle { x: 0.0, y: 0.0, width: w as f32, height: h as f32 };
            d.draw_texture_pro(tex, src, dst, Vector2::zero(), 0.0, Color::WHITE);
        }

        // Title
        let title = &engine.manifest.title;
        d.draw_text(title, 40, 60, 32, Color::WHITE);

        // Menu items
        let start_y = h / 2 - (ITEMS.len() as i32 * 36) / 2;
        for (i, item) in ITEMS.iter().enumerate() {
            let y = start_y + i as i32 * 36;
            let color = if i == self.cursor { Color::YELLOW } else { Color::LIGHTGRAY };
            let prefix = if i == self.cursor { "> " } else { "  " };
            d.draw_text(&format!("{prefix}{item}"), 60, y, 22, color);
        }
    }
}
