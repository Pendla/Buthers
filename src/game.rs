use ggez::Context;
use ggez::event::{EventHandler, Keycode, Mod};
use ggez::error::GameResult;
use ggez::graphics::{self, Color};
use ggez::timer;

use input::InputHandler;
use entities::{Player, Renderable, Updateable, World};

pub struct Game {
    player: Player,
    world: World,
    input: InputHandler,
}

impl Game {
    pub fn new(window_dims: (u32, u32)) -> Self {
        Self {
            player: Player::new(150.0, window_dims.1 as f32 - 150.0, 20.0),
            world: World::new(),
            input: InputHandler::new(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, context: &mut Context) -> GameResult<()> {
        let target_fps = 60;

        while timer::check_update_time(context, target_fps) {
            let delta_time = 1.0 / target_fps as f32;
            self.world.update(&self.input.state, delta_time);
            self.player.update(&self.input.state, delta_time);
        }

        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> GameResult<()> {
        graphics::set_background_color(context, Color::from_rgb(0, 0, 0));
        graphics::clear(context);

        self.world.render(context);
        self.player.render(context);

        graphics::present(context);
        Ok(())
    }

    fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: Keycode,
        modifier: Mod,
        repeat: bool,
    ) {
        self.input
            .key_down_event(context, keycode, modifier, repeat);
    }

    fn key_up_event(
        &mut self,
        context: &mut Context,
        keycode: Keycode,
        modifier: Mod,
        repeat: bool,
    ) {
        self.input.key_up_event(context, keycode, modifier, repeat);
    }
}
