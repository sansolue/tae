use crate::world::{Action, World};

pub enum TriggerOutcome {
    StartDialogue(String),
    MapTransition { map: String, x: u32, y: u32 },
    None,
}

pub fn evaluate(action: &Action, world: &mut World) -> TriggerOutcome {
    match action {
        Action::Dialogue { id } => TriggerOutcome::StartDialogue(id.clone()),

        Action::MapTransition { target_map, target_x, target_y } => TriggerOutcome::MapTransition {
            map: target_map.clone(),
            x: *target_x,
            y: *target_y,
        },

        Action::SetFlag { flag } => {
            world.set_flag(flag);
            TriggerOutcome::None
        }

        Action::Conditional { flag, then } => {
            if world.has_flag(flag) {
                evaluate(then, world)
            } else {
                TriggerOutcome::None
            }
        }
    }
}
