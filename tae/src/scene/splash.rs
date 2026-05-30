use raylib::prelude::*;

use super::{EngineState, Scene, SceneTransition};
use crate::asset;
use crate::world::SplashConfig;

pub struct SplashScene {
    config: SplashConfig,
    texture: Option<Texture2D>,
    timer: f32,
    loaded: bool,
}

impl SplashScene {
    pub fn new(config: SplashConfig) -> Self {
        Self { config, texture: None, timer: 0.0, loaded: false }
    }
}

impl Scene for SplashScene {
    fn on_load(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, engine: &EngineState) {
        if self.loaded { return; }
        if let Some(path) = &self.config.image.clone() {
            self.texture = asset::load_texture(rl, thread, &engine.store, &path);
        }
        self.loaded = true;
    }

    fn update(&mut self, rl: &mut RaylibHandle, _engine: &mut EngineState) -> SceneTransition {
        self.timer += rl.get_frame_time();
        let timed_out = self.config.duration > 0.0 && self.timer >= self.config.duration;
        let key_pressed = rl.get_key_pressed().is_some();
        if timed_out || key_pressed {
            return SceneTransition::Replace(Box::new(super::main_menu::MainMenuScene::new()));
        }
        SceneTransition::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState) {
        let w = engine.manifest.window_w as i32;
        let h = engine.manifest.window_h as i32;
        d.clear_background(Color::BLACK);
        if let Some(tex) = &self.texture {
            let src = Rectangle { x: 0.0, y: 0.0, width: tex.width as f32, height: tex.height as f32 };
            let dst = Rectangle { x: 0.0, y: 0.0, width: w as f32, height: h as f32 };
            d.draw_texture_pro(tex, src, dst, Vector2::zero(), 0.0, Color::WHITE);
        }
    }
}
