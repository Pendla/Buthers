use ggez::Context;
use ggez::event::{EventHandler, Keycode, Mod};
use ggez::error::GameResult;
use ggez::graphics::{self, Color};
use ggez::timer;

use models::{Movable, Player};

const MOVE_SPEED: f32 = 100.0;

pub struct KeyboardState {
    w: bool,
    a: bool,
    s: bool,
    d: bool,
}

impl KeyboardState {
    pub fn new() -> Self {
        Self {
            w: false,
            a: false,
            s: false,
            d: false,
        }
    }
}

pub struct Game {
    player: Player,
    keyboard_state: KeyboardState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            player: Player::new(0.0, 0.0, 20.0),
            keyboard_state: KeyboardState::new(),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let target_fps = 60;
        while timer::check_update_time(ctx, target_fps) {
            let dt = 1.0 / target_fps as f32;
            if self.keyboard_state.w {
                self.player.move_by(0.0, -MOVE_SPEED * dt);
            }
            if self.keyboard_state.a {
                self.player.move_by(-MOVE_SPEED * dt, 0.0);
            }
            if self.keyboard_state.s {
                self.player.move_by(0.0, MOVE_SPEED * dt);
            }
            if self.keyboard_state.d {
                self.player.move_by(MOVE_SPEED * dt, 0.0);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, Color::from_rgb(0, 0, 0));
        graphics::clear(ctx);

        graphics::set_color(ctx, Color::from_rgb(255, 0, 0));
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(
                self.player.pos.x,
                self.player.pos.y,
                self.player.width,
                self.player.height,
            ),
        );

        graphics::present(ctx);

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        match keycode {
            Keycode::W => self.keyboard_state.w = true,
            Keycode::A => self.keyboard_state.a = true,
            Keycode::S => self.keyboard_state.s = true,
            Keycode::D => self.keyboard_state.d = true,
            Keycode::Escape => ctx.quit().unwrap(),
            _ => (),
        }
    }

    fn key_up_event(&mut self, ctx: &mut Context, keycode: Keycode, keymod: Mod, repeat: bool) {
        match keycode {
            Keycode::W => self.keyboard_state.w = false,
            Keycode::A => self.keyboard_state.a = false,
            Keycode::S => self.keyboard_state.s = false,
            Keycode::D => self.keyboard_state.d = false,
            _ => ()
        }
    }
}
