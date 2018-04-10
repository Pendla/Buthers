use ggez::Context;
use ggez::event::{Keycode, Mod};

pub struct InputState {
    pub right: bool,
    pub left: bool,
    pub jump: bool,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            right: false,
            left: false,
            jump: false,
        }
    }
}

pub struct InputHandler {
    pub state: InputState,
}

impl InputHandler {
    pub fn new() -> Self {
        Self {
            state: InputState::new(),
        }
    }

    pub fn key_down_event(
        &mut self,
        context: &mut Context,
        keycode: Keycode,
        _modifier: Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::Right => self.state.right = true,
            Keycode::Left => self.state.left = true,
            Keycode::Space => self.state.jump = true,
            Keycode::Escape => context.quit().unwrap(),
            _ => {}
        }
    }

    pub fn key_up_event(
        &mut self,
        _context: &mut Context,
        keycode: Keycode,
        _modifier: Mod,
        _repeat: bool,
    ) {
        match keycode {
            Keycode::Right => self.state.right = false,
            Keycode::Left => self.state.left = false,
            Keycode::Space => self.state.jump = false,
            _ => {}
        }
    }
}
