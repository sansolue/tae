use std::path::Path;

use anyhow::Result;
use raylib::prelude::KeyboardKey;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(default)]
    pub fullscreen: bool,
    #[serde(default = "default_resolution")]
    pub resolution: [u32; 2],
    #[serde(default = "default_volume")]
    pub volume: u8,
    #[serde(default)]
    pub bindings: KeyBindings,
}

fn default_resolution() -> [u32; 2] {
    [640, 480]
}
fn default_volume() -> u8 {
    80
}

impl Default for Config {
    fn default() -> Self {
        Self {
            fullscreen: false,
            resolution: default_resolution(),
            volume: default_volume(),
            bindings: KeyBindings::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KeyBindings {
    pub move_up: String,
    pub move_down: String,
    pub move_left: String,
    pub move_right: String,
    pub confirm: String,
}

impl Default for KeyBindings {
    fn default() -> Self {
        Self {
            move_up: "Up".into(),
            move_down: "Down".into(),
            move_left: "Left".into(),
            move_right: "Right".into(),
            confirm: "Enter".into(),
        }
    }
}

impl Config {
    pub fn load(path: &Path) -> Self {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}

pub fn key_from_name(name: &str) -> KeyboardKey {
    match name {
        "Up" => KeyboardKey::KEY_UP,
        "Down" => KeyboardKey::KEY_DOWN,
        "Left" => KeyboardKey::KEY_LEFT,
        "Right" => KeyboardKey::KEY_RIGHT,
        "Enter" => KeyboardKey::KEY_ENTER,
        "Space" => KeyboardKey::KEY_SPACE,
        "W" => KeyboardKey::KEY_W,
        "A" => KeyboardKey::KEY_A,
        "S" => KeyboardKey::KEY_S,
        "D" => KeyboardKey::KEY_D,
        "Escape" => KeyboardKey::KEY_ESCAPE,
        _ => KeyboardKey::KEY_NULL,
    }
}

pub fn key_to_name(key: KeyboardKey) -> &'static str {
    match key {
        KeyboardKey::KEY_UP => "Up",
        KeyboardKey::KEY_DOWN => "Down",
        KeyboardKey::KEY_LEFT => "Left",
        KeyboardKey::KEY_RIGHT => "Right",
        KeyboardKey::KEY_ENTER => "Enter",
        KeyboardKey::KEY_SPACE => "Space",
        KeyboardKey::KEY_W => "W",
        KeyboardKey::KEY_A => "A",
        KeyboardKey::KEY_S => "S",
        KeyboardKey::KEY_D => "D",
        KeyboardKey::KEY_ESCAPE => "Escape",
        _ => "Unknown",
    }
}
