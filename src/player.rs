use crate::world::World;

pub struct Player {
    pub x: u32,
    pub y: u32,
}

impl Player {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    /// Attempt to move by (dx, dy). Returns true if the move succeeded.
    pub fn try_move(&mut self, dx: i32, dy: i32, world: &World) -> bool {
        let nx = self.x as i32 + dx;
        let ny = self.y as i32 + dy;
        if nx < 0 || ny < 0 {
            return false;
        }
        let nx = nx as u32;
        let ny = ny as u32;
        if world.is_wall(nx, ny) {
            return false;
        }
        self.x = nx;
        self.y = ny;
        true
    }
}
