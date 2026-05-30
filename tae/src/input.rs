use raylib::prelude::*;

use crate::config::{KeyBindings, key_from_name};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Confirm,
    Back,
    None,
}

pub fn poll(rl: &RaylibHandle, bindings: &KeyBindings) -> Intent {
    let up = key_from_name(&bindings.move_up);
    let down = key_from_name(&bindings.move_down);
    let left = key_from_name(&bindings.move_left);
    let right = key_from_name(&bindings.move_right);
    let confirm = key_from_name(&bindings.confirm);

    if rl.is_key_pressed(up) {
        Intent::MoveUp
    } else if rl.is_key_pressed(down) {
        Intent::MoveDown
    } else if rl.is_key_pressed(left) {
        Intent::MoveLeft
    } else if rl.is_key_pressed(right) {
        Intent::MoveRight
    } else if rl.is_key_pressed(confirm) {
        Intent::Confirm
    } else if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
        Intent::Back
    } else {
        Intent::None
    }
}
