use ::opengl_graphics::{ GlGraphics };
use ::piston::input::{ RenderArgs, UpdateArgs, ButtonArgs, Button, ButtonState};
use ::piston::input::keyboard::Key;
use ::models::{ Player, Movable };

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
    keyboard_state: KeyboardState
}


impl Game {
    pub fn new(gl: GlGraphics) -> Game {
        Game {
            gl: gl,
            player: Player::new(0.0, 0.0, 16),
            keyboard_state: KeyboardState::new()
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::{ Transformed, clear, rectangle };
        let x = self.player.pos.x;
        let y = self.player.pos.y;

        self.gl.draw(args.viewport(), |ctx, opengl| {
            let transform = ctx.transform.trans(x, y);
            clear([0.0, 0.0, 0.0, 1.0], opengl);
            rectangle(
                [1.0, 0.0, 1.0, 1.0],
                [0.0, 0.0, 16.0, 16.0],
                transform,
                opengl
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
        match args.button {
            Button::Keyboard(Key::D) =>  {
                match args.state {
                    ButtonState::Press => {
                        self.keyboard_state.d = true;
                    },
                    ButtonState::Release => {
                        self.keyboard_state.d = false;
                    }
                }
            },
            Button::Keyboard(Key::W) =>  {
                match args.state {
                    ButtonState::Press => {
                        self.keyboard_state.w = true;
                    },
                    ButtonState::Release => {
                        self.keyboard_state.w = false;
                    }
                }
            },
            Button::Keyboard(Key::A) =>  {
                match args.state {
                    ButtonState::Press => {
                        self.keyboard_state.a = true;
                    },
                    ButtonState::Release => {
                        self.keyboard_state.a = false;
                    }
                }
            },
            Button::Keyboard(Key::S) =>  {
                match args.state {
                    ButtonState::Press => {
                        self.keyboard_state.s = true;
                    },
                    ButtonState::Release => {
                        self.keyboard_state.s = false;
                    }
                }
            },
            _ => {}
        }
    }
}
