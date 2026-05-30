use crate::world::{Action, World};

pub enum TriggerOutcome {
    StartDialogue { id: String, then_set_flag: Option<String> },
    MapTransition { map: String, x: u32, y: u32, then_set_flag: Option<String> },
    None,
}

pub fn evaluate(action: &Action, world: &mut World) -> TriggerOutcome {
    match action {
        Action::Dialogue { id, then_set_flag } => TriggerOutcome::StartDialogue {
            id: id.clone(),
            then_set_flag: then_set_flag.clone(),
        },

        Action::MapTransition { target_map, target_x, target_y, then_set_flag } => {
            TriggerOutcome::MapTransition {
                map: target_map.clone(),
                x: *target_x,
                y: *target_y,
                then_set_flag: then_set_flag.clone(),
            }
        }

        Action::SetFlag { flag } => {
            world.set_flag(flag);
            TriggerOutcome::None
        }
    }
}
