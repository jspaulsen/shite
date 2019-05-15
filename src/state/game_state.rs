use nphysics2d::object::ColliderHandle;
use sdl2::event::Event;

use crate::engine::Context;


pub trait GameState: GameInputHandler + GamePhysicsHandler {
    fn update(&mut self, context: &mut Context) -> Result<(), String>;
    fn draw(&mut self, context: &mut Context) -> Result<(), String>;
}

// TODO(#25): [GameEventHandler] use -> Result<(), String>;
pub trait GameInputHandler {
    fn on_key_down(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_key_up(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_motion(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_button_down(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_button_up(&mut self, _context: &mut Context, _event: &Event) -> Result<(), String> {
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
