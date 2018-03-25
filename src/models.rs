
pub trait Movable {
    fn move_by(&mut self, x: f64, y: f64);
    fn move_to(&mut self, x: f64, y: f64);
}

#[derive(Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64
}

#[derive(Debug)]
pub struct Player {
    pub width: usize,
    pub height: usize,
    pub pos: Position,
}

impl Player {
    pub fn new(x: f64, y: f64, size: usize) -> Self {
        Self {
            width: size,
            height: size,
            pos: Position {x: x, y: y}
        }
    }
}

impl Movable for Player {
    fn move_by(&mut self, x: f64, y: f64) {
        self.pos.x += x;
        self.pos.y += y;
    }

    fn move_to(&mut self, x: f64, y: f64) {
        self.pos.x = x;
        self.pos.y = y;
    }
}
