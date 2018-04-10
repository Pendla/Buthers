extern crate ggez;

use ggez::conf;
use ggez::Context;
use ggez::event;

mod entities;
mod game;
mod input;

use game::Game;

const WINDOW_DIMS: (u32, u32) = (1280, 720);

fn main() {
    let mut config = conf::Conf::new();
    config.window_mode.width = WINDOW_DIMS.0;
    config.window_mode.height = WINDOW_DIMS.1;
    let context = &mut Context::load_from_conf("Buthers", "Simon & Pat", config).unwrap();
    let game = &mut Game::new(WINDOW_DIMS);
    event::run(context, game).unwrap();
}
