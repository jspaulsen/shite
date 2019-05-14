use nphysics2d::object::ColliderHandle;
use sdl2::event::Event;

use crate::engine::Context;


pub type BoxedGameState = Box<GameState>;


pub trait GameState: GameInputHandler + GamePhysicsHandler {
    fn update(&mut self, context: &mut Context) -> Result<(), String>;
    fn draw(&mut self, context: &mut Context) -> Result<(), String>;
}

// TODO(#25): [GameEventHandler] use -> Result<(), String>;
pub trait GameInputHandler {
    fn on_key_down(&mut self, context: &mut Context, event: &Event);
    fn on_key_up(&mut self, context: &mut Context, event: &Event);
    fn on_mouse_motion(&mut self, context: &mut Context, event: &Event);
    fn on_mouse_button_down(&mut self, context: &mut Context, event: &Event);
    fn on_mouse_button_up(&mut self, context: &mut Context, event: &Event);
    fn on_mouse_wheel(&mut self, context: &mut Context, event: &Event);
}

pub trait GamePhysicsHandler {
    fn on_collision_start(&mut self, context: &mut Context, coh1: ColliderHandle, coh2: ColliderHandle);
    fn on_collision_end(&mut self, context: &mut Context, coh1: ColliderHandle, coh2: ColliderHandle);
}
