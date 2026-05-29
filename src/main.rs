mod archive;
mod dialogue;
mod input;
mod loader;
mod player;
mod renderer;
mod trigger;
mod world;

use std::env;
use std::path::PathBuf;

use anyhow::Result;
use dialogue::DialogueState;
use input::Intent;
use player::Player;
use trigger::TriggerOutcome;
use world::World;

fn exe_dir() -> PathBuf {
    env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn main() -> Result<()> {
    let base = exe_dir();
    let store = archive::load(&base)?;

    let manifest = loader::load_manifest(&store)?;
    let start_map = loader::load_map(&store, &manifest.start_map.clone())?;
    let npcs = loader::load_npcs(&store)?;
    let dialogues = loader::load_dialogues(&store)?;

    let start_x = 1u32;
    let start_y = 1u32;

    let mut world = World::new(manifest, start_map, npcs, dialogues);
    let mut player = Player::new(start_x, start_y);
    let mut active_dialogue: Option<DialogueState> = None;

    let (mut rl, thread) = raylib::init()
        .size(world.manifest.window_w as i32, world.manifest.window_h as i32)
        .title(&world.manifest.title.clone())
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let intent = input::poll(&rl);

        // --- Update ---
        if let Some(ref mut dlg) = active_dialogue {
            if intent == Intent::Confirm {
                if !dlg.advance() {
                    let flag = dlg.then_set_flag.clone();
                    active_dialogue = None;
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
                let moved = player.try_move(dx, dy, &world);
                if moved {
                    // Clone trigger action to avoid borrow conflict with world mutation
                    if let Some(action) = world
                        .trigger_at(player.x, player.y)
                        .map(|t| t.action.clone())
                    {
                        match trigger::evaluate(&action, &mut world) {
                            TriggerOutcome::StartDialogue { id, then_set_flag } => {
                                if let Some(def) = world.dialogues.get(&id) {
                                    active_dialogue = Some(DialogueState::start(def, then_set_flag));
                                }
                            }
                            TriggerOutcome::MapTransition { map, x, y, then_set_flag } => {
                                let new_map = loader::load_map(&store, &map)?;
                                world.current_map = new_map;
                                player.x = x;
                                player.y = y;
                                if let Some(flag) = then_set_flag {
                                    world.set_flag(&flag);
                                }
                            }
                            TriggerOutcome::None => {}
                        }
                    }
                }
            }
        }

        // --- Draw ---
        let mut d = rl.begin_drawing(&thread);
        renderer::draw(&mut d, &world, &player, active_dialogue.as_ref());
    }

    Ok(())
}
