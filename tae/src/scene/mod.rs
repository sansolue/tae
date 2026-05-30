pub mod about;
pub mod exit_confirm;
pub mod game;
pub mod main_menu;
pub mod save_load;
pub mod settings;
pub mod splash;

use raylib::prelude::*;
use tae_core::FileStore;

use crate::config::Config;
use crate::save::SaveManager;
use crate::world::GameManifest;

pub struct EngineState {
    pub store: FileStore,
    pub config: Config,
    pub saves: SaveManager,
    pub manifest: GameManifest,
}

pub enum SceneTransition {
    None,
    Push(Box<dyn Scene>),
    Replace(Box<dyn Scene>),
    Pop,
    Quit,
}

pub trait Scene {
    /// Called once when the scene is first entered, while we still hold &mut RaylibHandle.
    fn on_load(&mut self, _rl: &mut RaylibHandle, _thread: &RaylibThread, _engine: &EngineState) {}
    fn update(&mut self, rl: &mut RaylibHandle, engine: &mut EngineState) -> SceneTransition;
    fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState);
    /// If true, the scene below this one is also drawn (overlay scenes).
    fn is_overlay(&self) -> bool { false }
}

pub struct SceneStack {
    stack: Vec<Box<dyn Scene>>,
}

impl SceneStack {
    pub fn new(initial: Box<dyn Scene>) -> Self {
        Self { stack: vec![initial] }
    }

    /// Call after pushing or replacing to trigger asset loading.
    pub fn load_top(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread, engine: &EngineState) {
        if let Some(scene) = self.stack.last_mut() {
            scene.on_load(rl, thread, engine);
        }
    }

    /// Returns false if the engine should quit.
    pub fn update(&mut self, rl: &mut RaylibHandle, engine: &mut EngineState) -> bool {
        let transition = match self.stack.last_mut() {
            Some(scene) => scene.update(rl, engine),
            None => return false,
        };
        match transition {
            SceneTransition::None => {}
            SceneTransition::Push(s) => self.stack.push(s),
            SceneTransition::Replace(s) => {
                self.stack.pop();
                self.stack.push(s);
            }
            SceneTransition::Pop => {
                self.stack.pop();
            }
            SceneTransition::Quit => return false,
        }
        !self.stack.is_empty()
    }

    /// Load assets for the new top scene after a transition.
    /// Call this after update() if the stack may have changed.
    pub fn load_top_if_needed(
        &mut self,
        rl: &mut RaylibHandle,
        thread: &RaylibThread,
        engine: &EngineState,
    ) {
        if let Some(scene) = self.stack.last_mut() {
            // Scenes use a loaded flag internally; on_load is idempotent
            scene.on_load(rl, thread, engine);
        }
    }

    pub fn draw(&self, d: &mut RaylibDrawHandle, engine: &EngineState) {
        let n = self.stack.len();
        if n == 0 { return; }
        if n >= 2 && self.stack[n - 1].is_overlay() {
            self.stack[n - 2].draw(d, engine);
        }
        self.stack[n - 1].draw(d, engine);
    }
}
