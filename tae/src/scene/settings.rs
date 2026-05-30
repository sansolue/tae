use raylib::prelude::*;

use super::{EngineState, Scene, SceneTransition};
use crate::config::key_to_name;

const RESOLUTIONS: &[[u32; 2]] = &[
    [640, 480],
    [800, 600],
    [1024, 768],
    [1280, 720],
    [1920, 1080],
];

#[derive(Clone, Copy, PartialEq)]
enum SettingItem {
    Resolution,
    Fullscreen,
    Volume,
    BindMoveUp,
    BindMoveDown,
    BindMoveLeft,
    BindMoveRight,
    BindConfirm,
    ResetDefaults,
}

const ITEMS: &[SettingItem] = &[
    SettingItem::Resolution,
    SettingItem::Fullscreen,
    SettingItem::Volume,
    SettingItem::BindMoveUp,
    SettingItem::BindMoveDown,
    SettingItem::BindMoveLeft,
    SettingItem::BindMoveRight,
    SettingItem::BindConfirm,
    SettingItem::ResetDefaults,
];

pub struct SettingsScene {
    cursor: usize,
    capturing: bool, // waiting for a keypress to rebind
}

impl SettingsScene {
    pub fn new() -> Self {
        Self { cursor: 0, capturing: false }
    }

    fn item_label(item: SettingItem, engine: &EngineState) -> String {
        let cfg = &engine.config;
        match item {
            SettingItem::Resolution => {
                let r = cfg.resolution;
                format!("Resolution:  {}x{}", r[0], r[1])
            }
            SettingItem::Fullscreen => {
                format!("Fullscreen:  {}", if cfg.fullscreen { "On" } else { "Off" })
            }
            SettingItem::Volume => format!("Volume:      {}%", cfg.volume),
            SettingItem::BindMoveUp => format!("Move Up:     {}", cfg.bindings.move_up),
            SettingItem::BindMoveDown => format!("Move Down:   {}", cfg.bindings.move_down),
            SettingItem::BindMoveLeft => format!("Move Left:   {}", cfg.bindings.move_left),
            SettingItem::BindMoveRight => format!("Move Right:  {}", cfg.bindings.move_right),
            SettingItem::BindConfirm => format!("Confirm:     {}", cfg.bindings.confirm),
            SettingItem::ResetDefaults => "Reset to Defaults".to_string(),
        }
    }
}

impl Scene for SettingsScene {
    fn update(&mut self, rl: &mut RaylibHandle, engine: &mut EngineState) -> SceneTransition {
        // Key binding capture mode
        if self.capturing {
            if let Some(key) = rl.get_key_pressed() {
                if key != KeyboardKey::KEY_ESCAPE {
                    let name = key_to_name(key).to_string();
                    let b = &mut engine.config.bindings;
                    match ITEMS[self.cursor] {
                        SettingItem::BindMoveUp => b.move_up = name,
                        SettingItem::BindMoveDown => b.move_down = name,
                        SettingItem::BindMoveLeft => b.move_left = name,
                        SettingItem::BindMoveRight => b.move_right = name,
                        SettingItem::BindConfirm => b.confirm = name,
                        _ => {}
                    }
                }
                self.capturing = false;
            }
            return SceneTransition::None;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            let _ = save_config(engine);
            return SceneTransition::Pop;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
            self.cursor = self.cursor.saturating_sub(1);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S) {
            self.cursor = (self.cursor + 1).min(ITEMS.len() - 1);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ENTER) || rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            match ITEMS[self.cursor] {
                SettingItem::Resolution => {
                    let cur = RESOLUTIONS.iter().position(|r| *r == engine.config.resolution).unwrap_or(0);
                    engine.config.resolution = RESOLUTIONS[(cur + 1) % RESOLUTIONS.len()];
                }
                SettingItem::Fullscreen => {
                    engine.config.fullscreen = !engine.config.fullscreen;
                }
                SettingItem::Volume => {
                    engine.config.volume = (engine.config.volume + 10) % 110;
                }
                SettingItem::BindMoveUp
                | SettingItem::BindMoveDown
                | SettingItem::BindMoveLeft
                | SettingItem::BindMoveRight
                | SettingItem::BindConfirm => {
                    self.capturing = true;
                }
                SettingItem::ResetDefaults => {
                    engine.config = crate::config::Config::default();
                }
            }
        }

        // Volume adjustment with left/right
        if ITEMS[self.cursor] == SettingItem::Volume {
            if rl.is_key_pressed(KeyboardKey::KEY_LEFT) && engine.config.volume >= 10 {
                engine.config.volume -= 10;
            }
            if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) && engine.config.volume <= 90 {
                engine.config.volume += 10;
            }
        }

        SceneTransition::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState) {
        let w = engine.manifest.window_w as i32;
        let h = engine.manifest.window_h as i32;
        d.clear_background(Color { r: 15, g: 15, b: 25, a: 255 });
        d.draw_text("Settings", 40, 30, 28, Color::WHITE);
        d.draw_line(40, 65, w - 40, 65, Color::GRAY);

        let start_y = 80;
        for (i, &item) in ITEMS.iter().enumerate() {
            let y = start_y + i as i32 * 32;
            let is_sel = i == self.cursor;
            let label = Self::item_label(item, engine);
            let color = if is_sel { Color::YELLOW } else { Color::LIGHTGRAY };
            let prefix = if is_sel { "> " } else { "  " };
            d.draw_text(&format!("{prefix}{label}"), 40, y, 18, color);
        }

        if self.capturing {
            d.draw_rectangle(0, h - 40, w, 40, Color { r: 0, g: 0, b: 0, a: 200 });
            d.draw_text("Press any key to bind (Escape to cancel)...", 20, h - 28, 14, Color::YELLOW);
        } else {
            d.draw_text("Escape to save & return", 20, h - 24, 12, Color::GRAY);
        }
    }
}

fn save_config(engine: &EngineState) -> anyhow::Result<()> {
    if let Some(data_dir) = user_data_dir(&engine.manifest.title) {
        engine.config.save(&data_dir.join("config.toml"))?;
    }
    Ok(())
}

fn user_data_dir(title: &str) -> Option<std::path::PathBuf> {
    dirs::data_dir().map(|d| d.join("tae").join(title))
}
