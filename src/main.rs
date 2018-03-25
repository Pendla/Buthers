extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod models;
mod game;


use piston::input::{ RenderEvent, UpdateEvent, ButtonEvent};
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ OpenGL, GlGraphics };
use piston::event_loop::{Events, EventSettings};
use piston::window::WindowSettings;

use game::Game;

const WINDOW_DIMS: [u32; 2] = [1280, 720];

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new("buthers", WINDOW_DIMS)
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = Game::new(GlGraphics::new(opengl));

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }
        if let Some(u) = e.update_args() {
            app.update(&u);
        }
        if let Some(b) = e.button_args() {
            app.on_keypress(&b);
        }
    }
}
