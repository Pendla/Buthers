use ::opengl_graphics::{ GlGraphics };
use ::piston::input::{ RenderArgs, UpdateArgs };

use ::models::{ Player, Movable };

pub struct Game {
    gl: GlGraphics,
    player: Player,
}


impl Game {
    pub fn new(gl: GlGraphics) -> Game {
        Game {
            gl: gl,
            player: Player::new(0.0, 0.0, 16)
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
        self.player.move_by(args.dt * 10.0, args.dt * 10.0)
    }
}
