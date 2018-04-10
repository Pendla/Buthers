use entities::Updateable;
use ggez::Context;
use ggez::graphics::{self, Color};

use entities::{Movable, Position, Renderable};
use input::InputState;

const MOVE_SPEED: f32 = 150.0;

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

impl Renderable for Player {
    fn render(&mut self, ctx: &mut Context) {
        graphics::set_color(ctx, Color::from_rgb(255, 0, 0)).unwrap();
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(self.pos.x, self.pos.y, self.width, self.height),
        ).unwrap();
    }
}

impl Updateable for Player {
    fn update(&mut self, input: &InputState, delta_time: f32) {
        if input.right {
            self.move_by(MOVE_SPEED * delta_time, 0.0);
        }

        if input.left {
            self.move_by(-MOVE_SPEED * delta_time, 0.0);
        }

        if input.jump {
            self.move_by(0.0, -MOVE_SPEED * delta_time);
        }
    }
}
