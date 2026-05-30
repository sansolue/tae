use raylib::prelude::*;

use super::{EngineState, Scene, SceneTransition};
use crate::asset;

pub struct AboutScene {
    texture: Option<Texture2D>,
    loaded: bool,
}

impl AboutScene {
    pub fn new() -> Self {
        Self { texture: None, loaded: false }
    }
}

impl Scene for AboutScene {
    fn on_load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, engine: &EngineState) {
        if self.loaded { return; }
        if let Some(path) = &engine.manifest.about.image.clone() {
            self.texture = asset::load_texture(rl, thread, &engine.store, &path);
        }
        self.loaded = true;
    }

    fn update(&mut self, rl: &mut RaylibHandle, _engine: &mut EngineState) -> SceneTransition {
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE)
            || rl.is_key_pressed(KeyboardKey::KEY_ENTER)
            || rl.is_key_pressed(KeyboardKey::KEY_SPACE)
        {
            return SceneTransition::Pop;
        }
        SceneTransition::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState) {
        let w = engine.manifest.window_w as i32;
        let h = engine.manifest.window_h as i32;
        d.clear_background(Color { r: 10, g: 10, b: 30, a: 255 });
        if let Some(tex) = &self.texture {
            let src = Rectangle { x: 0.0, y: 0.0, width: tex.width as f32, height: tex.height as f32 };
            let dst = Rectangle { x: 0.0, y: 0.0, width: w as f32, height: h as f32 };
            d.draw_texture_pro(tex, src, dst, Vector2::zero(), 0.0, Color::WHITE);
        } else if let Some(text) = &engine.manifest.about.text {
            d.draw_text(text, 40, 40, 16, Color::WHITE);
        } else {
            d.draw_text(&engine.manifest.title, 40, 40, 24, Color::WHITE);
        }
        d.draw_text("Press Enter or Escape to return", 16, h - 28, 12, Color::GRAY);
    }
}
