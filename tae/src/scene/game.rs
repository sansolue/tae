use raylib::prelude::*;

use super::{EngineState, Scene, SceneTransition};
use crate::dialogue::DialogueState;
use crate::input;
use crate::input::Intent;
use crate::loader;
use crate::player::Player;
use crate::renderer;
use crate::trigger::TriggerOutcome;
use crate::world::World;

pub struct GameScene {
    world: Option<World>,
    player: Option<Player>,
    active_dialogue: Option<DialogueState>,
}

impl GameScene {
    pub fn new() -> Self {
        Self { world: None, player: None, active_dialogue: None }
    }

    fn init(&mut self, engine: &EngineState) -> anyhow::Result<()> {
        let m = &engine.manifest;
        let start_map = loader::load_map(&engine.store, &m.start_map)?;
        let npcs = loader::load_npcs(&engine.store)?;
        let dialogues = loader::load_dialogues(&engine.store)?;
        self.world = Some(World::new(m.clone(), start_map, npcs, dialogues));
        self.player = Some(Player::new(1, 1));
        Ok(())
    }
}

impl Scene for GameScene {
    fn update(&mut self, rl: &mut RaylibHandle, engine: &mut EngineState) -> SceneTransition {
        // Lazy init
        if self.world.is_none() {
            if let Err(e) = self.init(engine) {
                eprintln!("GameScene init error: {e}");
                return SceneTransition::Pop;
            }
        }

        let world = self.world.as_mut().unwrap();
        let player = self.player.as_mut().unwrap();
        let intent = input::poll(rl, &engine.config.bindings);

        if intent == Intent::Back {
            // Pause menu / back to main menu (future: push PauseScene)
            return SceneTransition::None;
        }

        if let Some(ref mut dlg) = self.active_dialogue {
            if intent == Intent::Confirm {
                if !dlg.advance() {
                    let flag = dlg.then_set_flag.clone();
                    self.active_dialogue = None;
                    if let Some(f) = flag {
                        world.set_flag(&f);
                    }
                }
            }
        } else {
            let (dx, dy) = match intent {
                Intent::MoveUp => (0, -1),
                Intent::MoveDown => (0, 1),
                Intent::MoveLeft => (-1, 0),
                Intent::MoveRight => (1, 0),
                _ => (0, 0),
            };

            if dx != 0 || dy != 0 {
                let moved = player.try_move(dx, dy, world);
                if moved {
                    if let Some(action) = world.trigger_at(player.x, player.y).map(|t| t.action.clone()) {
                        match crate::trigger::evaluate(&action, world) {
                            TriggerOutcome::StartDialogue { id, then_set_flag } => {
                                if let Some(def) = world.dialogues.get(&id) {
                                    self.active_dialogue = Some(DialogueState::start(def, then_set_flag));
                                }
                            }
                            TriggerOutcome::MapTransition { map, x, y, then_set_flag } => {
                                if let Ok(new_map) = loader::load_map(&engine.store, &map) {
                                    world.current_map = new_map;
                                    player.x = x;
                                    player.y = y;
                                    if let Some(flag) = then_set_flag {
                                        world.set_flag(&flag);
                                    }
                                }
                            }
                            TriggerOutcome::None => {}
                        }
                    }
                }
            }
        }

        SceneTransition::None
    }

    fn draw(&self, d: &mut RaylibDrawHandle, _engine: &EngineState) {
        if let (Some(world), Some(player)) = (&self.world, &self.player) {
            renderer::draw(d, world, player, self.active_dialogue.as_ref());
        } else {
            d.clear_background(Color::BLACK);
        }
    }
}
