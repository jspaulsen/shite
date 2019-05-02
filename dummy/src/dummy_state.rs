use sdl2::event::Event;
use shite::game::{GameState, GameEventHandler, BoxedRenderable};

pub struct DummyState {

}

// GameEventHandler
impl GameEventHandler for DummyState {
    fn on_key_down(&mut self, event: &Event) {
        println!("on_key_down: {:?}", event);
    }

    fn on_key_up(&mut self, event: &Event) {
        println!("on_key_up: {:?}", event);
    }

    fn on_mouse_motion(&mut self, event: &Event) {
        println!("on_mouse_motion: {:?}", event);
    }

    fn on_mouse_button_down(&mut self, event: &Event) {
        println!("on_mouse_button_down: {:?}", event);
    }

    fn on_mouse_button_up(&mut self, event: &Event) {
        println!("on_mouse_button_up: {:?}", event);
    }

    fn on_mouse_wheel(&mut self, event: &Event) {
        println!("on_mouse_wheel: {:?}", event);
    }
}

impl GameState for DummyState {
    fn get_state_name(&self) -> String {
        return "DummyState".to_string();
    }
}
