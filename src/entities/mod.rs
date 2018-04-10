use ggez::Context;

use input::InputState;

pub use self::player::Player;
mod player;

pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub trait Movable {
    fn move_by(&mut self, x: f32, y: f32);
    fn move_to(&mut self, x: f32, y: f32);
}

pub trait Renderable {
    fn render(&mut self, ctx: &mut Context);
}

pub trait Updateable {
    fn update(&mut self, input: &InputState, delta_time: f32);
}
