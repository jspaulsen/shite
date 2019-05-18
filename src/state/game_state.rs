use std::process::exit;

use nphysics2d::object::ColliderHandle;
use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;

use crate::engine::Context;


pub trait GameState: GameInputHandler + GamePhysicsHandler + GameApplicationHandler + GameWindowHandler {
    fn update(&mut self, context: &mut Context) -> Result<(), String>;
    fn draw(&mut self, context: &mut Context) -> Result<(), String>;
}

pub trait GameInputHandler {
    fn on_key_press(&mut self, _context: &mut Context, _event: &Event, _kbd_state: &KeyboardState) -> Result<(), String> {
        Ok(())
    }
    fn on_key_release(&mut self, _context: &mut Context, _event: &Event, _kbd_state: &KeyboardState) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_motion(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_click(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_click_release(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_wheel(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}

pub trait GamePhysicsHandler {
    fn on_collision_start(&mut self, _context: &mut Context, _coh1: ColliderHandle, _coh2: ColliderHandle) -> Result<(), String> {
        Ok(())
    }
    fn on_collision_end(&mut self, _context: &mut Context, _coh1: ColliderHandle, _coh2: ColliderHandle) -> Result<(), String> {
        Ok(())
    }
}

/// Higher level application handler
pub trait GameApplicationHandler {
    fn on_quit(&mut self, _context: &mut Context, _event: &Event) {
        exit(0);
    }

    /// Called whenever the operating system is terminating the application
    ///
    fn on_terminating(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_low_memory(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}

pub trait GameWindowHandler {
    fn on_will_enter_background(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_entered_background(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_will_enter_foreground(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_entered_foreground(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_window_event(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_text_editing(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_text_input(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
}
