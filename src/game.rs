use opengl_graphics::GlGraphics;
use piston::input::{Button, ButtonArgs, ButtonState, RenderArgs, UpdateArgs};
use piston::input::keyboard::Key;
use models::{Movable, Player};

const MOVE_SPEED: f64 = 100.0;

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
    gl: GlGraphics,
    player: Player,
    keyboard_state: KeyboardState,
}

impl Game {
    pub fn new(gl: GlGraphics) -> Game {
        Game {
            gl: gl,
            player: Player::new(0.0, 0.0, 16),
            keyboard_state: KeyboardState::new(),
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::{clear, rectangle, Transformed};
        let x = self.player.pos.x;
        let y = self.player.pos.y;

        self.gl.draw(args.viewport(), |ctx, opengl| {
            let transform = ctx.transform.trans(x, y);
            clear([0.0, 0.0, 0.0, 1.0], opengl);
            rectangle(
                [1.0, 0.0, 1.0, 1.0],
                [0.0, 0.0, 16.0, 16.0],
                transform,
                opengl,
            );
        })
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        let keys = &self.keyboard_state;
        let dt = args.dt;
        let move_size = dt * MOVE_SPEED;
        if keys.w {
            self.player.move_by(0.0, -move_size);
        }
        if keys.s {
            self.player.move_by(0.0, move_size);
        }
        if keys.a {
            self.player.move_by(-move_size, 0.0);
        }
        if keys.d {
            self.player.move_by(move_size, 0.0);
        }
    }

    pub fn on_keypress(&mut self, args: &ButtonArgs) {
        let state = args.state == ButtonState::Press;
        match args.button {
            Button::Keyboard(Key::D) => self.keyboard_state.d = state,
            Button::Keyboard(Key::W) => self.keyboard_state.w = state,
            Button::Keyboard(Key::A) => self.keyboard_state.a = state,
            Button::Keyboard(Key::S) => self.keyboard_state.s = state,
            _ => {}
        }
    }
}
