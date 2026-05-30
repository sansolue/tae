mod archive;
mod asset;
mod config;
mod dialogue;
mod input;
mod loader;
mod player;
mod renderer;
mod save;
mod scene;
mod trigger;
mod world;

use std::env;
use std::path::PathBuf;

use anyhow::Result;
use config::Config;
use save::SaveManager;
use scene::{EngineState, SceneStack};

fn exe_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn user_data_dir(title: &str) -> PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("tae")
        .join(title)
}

fn main() -> Result<()> {
    let base = exe_dir();
    let store = archive::load(&base)?;
    let manifest = loader::load_manifest(&store)?;

    let data_dir = user_data_dir(&manifest.title);
    let config = Config::load(&data_dir.join("config.toml"));
    let saves = SaveManager::new(data_dir.clone(), manifest.saves.slots_per_page);

    let (mut rl, thread) = raylib::init()
        .size(manifest.window_w as i32, manifest.window_h as i32)
        .title(&manifest.title.clone())
        .build();

    if config.fullscreen {
        rl.toggle_fullscreen();
    }

    rl.set_exit_key(None);
    rl.set_target_fps(60);

    let mut engine = EngineState { store, config, saves, manifest };

    // Start with Splash if configured, otherwise go straight to MainMenu
    let initial: Box<dyn scene::Scene> = if engine.manifest.splash.duration > 0.0
        || engine.manifest.splash.image.is_some()
    {
        let cfg = engine.manifest.splash.clone();
        Box::new(scene::splash::SplashScene::new(cfg))
    } else {
        Box::new(scene::main_menu::MainMenuScene::new())
    };

    let mut stack = SceneStack::new(initial);
    stack.load_top(&mut rl, &thread, &engine);

    while !rl.window_should_close() {
        let cmd = rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT_SUPER)
            || rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT_SUPER);

        if cmd && rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_Q) {
            break;
        }
        if cmd && rl.is_key_pressed(raylib::consts::KeyboardKey::KEY_ENTER) {
            rl.toggle_fullscreen();
        }

        let alive = stack.update(&mut rl, &mut engine);
        stack.load_top_if_needed(&mut rl, &thread, &engine);
        if !alive { break; }

        let mut d = rl.begin_drawing(&thread);
        stack.draw(&mut d, &engine);
    }

    Ok(())
}
