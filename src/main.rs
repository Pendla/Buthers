extern crate ggez;

use ggez::conf;
use ggez::Context;
use ggez::event;

mod models;
mod game;

use game::Game;

const WINDOW_DIMS: [u32; 2] = [1280, 720];

fn main() {
    let config = conf::Conf::new();
    let context = &mut Context::load_from_conf(
        "Buthers",
        "Simon & Pat",
        config
    ).unwrap();
    let game = &mut Game::new();
    event::run(context, game).unwrap();
}
