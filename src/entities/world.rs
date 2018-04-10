use ggez::Context;
use ggez::graphics::{self, Color};
use input::InputState;
use entities::{Renderable, Updateable};

pub struct World {}

impl World {
    pub fn new() -> Self {
        Self {}
    }
}

impl Renderable for World {
    fn render(&mut self, context: &mut Context) {
        graphics::set_color(context, Color::from_rgb(60, 60, 60)).unwrap();
        graphics::rectangle(
            context,
            graphics::DrawMode::Fill,
            graphics::Rect::new(0.0, 690.0, 1280.0, 30.0),
        ).unwrap();
    }
}

impl Updateable for World {
    fn update(&mut self, _input: &InputState, _delta_time: f32) {
        return;
    }
}
