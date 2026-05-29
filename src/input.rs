use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Intent {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Confirm,
    None,
}

pub fn poll(rl: &RaylibHandle) -> Intent {
    if rl.is_key_pressed(KeyboardKey::KEY_UP) || rl.is_key_pressed(KeyboardKey::KEY_W) {
        Intent::MoveUp
    } else if rl.is_key_pressed(KeyboardKey::KEY_DOWN) || rl.is_key_pressed(KeyboardKey::KEY_S) {
        Intent::MoveDown
    } else if rl.is_key_pressed(KeyboardKey::KEY_LEFT) || rl.is_key_pressed(KeyboardKey::KEY_A) {
        Intent::MoveLeft
    } else if rl.is_key_pressed(KeyboardKey::KEY_RIGHT) || rl.is_key_pressed(KeyboardKey::KEY_D) {
        Intent::MoveRight
    } else if rl.is_key_pressed(KeyboardKey::KEY_SPACE)
        || rl.is_key_pressed(KeyboardKey::KEY_ENTER)
    {
        Intent::Confirm
    } else {
        Intent::None
    }
}
