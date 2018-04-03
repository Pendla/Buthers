pub trait Movable {
    fn move_by(&mut self, x: f32, y: f32);
    fn move_to(&mut self, x: f32, y: f32);
}

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct Player {
    pub width: f32,
    pub height: f32,
    pub pos: Position,
}

impl Player {
    pub fn new(x: f32, y: f32, size: f32) -> Self {
        Self {
            width: size,
            height: size,
            pos: Position { x: x, y: y },
        }
    }
}

impl Movable for Player {
    fn move_by(&mut self, x: f32, y: f32) {
        self.pos.x += x;
        self.pos.y += y;
    }

    fn move_to(&mut self, x: f32, y: f32) {
        self.pos.x = x;
        self.pos.y = y;
    }
}
