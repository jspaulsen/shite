use std::process::exit;

use nphysics2d::object::ColliderHandle;
use sdl2::event::Event;
use sdl2::keyboard::KeyboardState;

use crate::engine::Context;
use crate::event::{
    GameApplicationEvent,
    GameKeyboardEvent,
    GameMouseButtonEvent,
    GameMouseMotionEvent,
    GameMouseWheelEvent,
    GameTextEditEvent,
    GameTextInputEvent,
    UiEvent,
};
use crate::ui::UiWindow;

pub trait GameState: GameInputHandler + GamePhysicsHandler + GameApplicationHandler + GameWindowHandler + GameUiEventHandler {
    fn update(&mut self, context: &mut Context) -> Result<(), String>;
    fn draw(&mut self, context: &mut Context) -> Result<(), String>;

    /// Returns a list of active UI elements.  This function should return _only_ the base objects
    /// and not children of objects.
    fn active_ui(&self) -> Option<&UiWindow> {
        None
    }

    fn active_ui_mut(&mut self) -> Option<&mut UiWindow> {
        None
    }
}

pub trait GameInputHandler {
    fn on_key_press(&mut self, _ctx: &mut Context, _evnt: &GameKeyboardEvent, _kbd: &KeyboardState) -> Result<(), String> {
        Ok(())
    }
    fn on_key_release(&mut self, _ctx: &mut Context, _evnt: &GameKeyboardEvent, _kbd: &KeyboardState) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_motion(&mut self, _ctx: &mut Context, _evnt: &GameMouseMotionEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_click(&mut self, _ctx: &mut Context, _evnt: &GameMouseButtonEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_click_release(&mut self, _ctx: &mut Context, _evnt: &GameMouseButtonEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_mouse_wheel(&mut self, _ctx: &mut Context, _evnt: &GameMouseWheelEvent) -> Result<(), String> {
        Ok(())
    }
}

pub trait GamePhysicsHandler {
    fn on_collision_start(&mut self, _ctx: &mut Context, _coh1: ColliderHandle, _coh2: ColliderHandle) -> Result<(), String> {
        Ok(())
    }
    fn on_collision_end(&mut self, _ctx: &mut Context, _coh1: ColliderHandle, _coh2: ColliderHandle) -> Result<(), String> {
        Ok(())
    }
}

/// Higher level application handler
pub trait GameApplicationHandler {
    fn on_quit(&mut self, _ctx: &mut Context, _evnt: &GameApplicationEvent) {
        exit(0);
    }

    /// Called whenever the operating system is terminating the application
    fn on_terminating(&mut self, _ctx: &mut Context, _evnt: &GameApplicationEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_low_memory(&mut self, _ctx: &mut Context, _evnt: &GameApplicationEvent) -> Result<(), String> {
        Ok(())
    }
}

pub trait GameWindowHandler {
    fn on_will_enter_background(&mut self, _ctx: &mut Context, _evnt: &GameApplicationEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_entered_background(&mut self, _ctx: &mut Context, _evnt: &GameApplicationEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_will_enter_foreground(&mut self, _ctx: &mut Context, _evnt: &GameApplicationEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_entered_foreground(&mut self, _ctx: &mut Context, _evnt: &GameApplicationEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_window_event(&mut self, _ctx: &mut Context, _evnt: &Event) -> Result<(), String> {
        Ok(())
    }
    fn on_text_editing(&mut self, _ctx: &mut Context, _evnt: &GameTextEditEvent) -> Result<(), String> {
        Ok(())
    }
    fn on_text_input(&mut self, _ctx: &mut Context, _evnt: &GameTextInputEvent) -> Result<(), String> {
        Ok(())
    }
}

pub trait GameUiEventHandler {
    fn on_ui_event(&mut self, _ctx: &mut Context, _evnt: UiEvent) -> Result<(), String> {
        Ok(())
    }
}
