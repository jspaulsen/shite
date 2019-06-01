use crate::event::{
    GameKeyboardEvent,
    GameMouseButtonEvent,
    GameMouseMotionEvent,
    UiEvent,
};

pub trait Interfaceable {
    fn on_key_press(&mut self, _event: &GameKeyboardEvent) -> Result<(bool, UiEvent), String> {
        Ok((false, UiEvent::None))
    }

    fn on_key_release(&mut self, _event: &GameKeyboardEvent) -> Result<(bool, UiEvent), String> {
        Ok((false, UiEvent::None))
    }

    fn on_click(&mut self, _event: &GameMouseButtonEvent) -> Result<(bool, UiEvent), String> {
        Ok((false, UiEvent::None))
    }

    fn on_click_release(&mut self, _event: &GameMouseButtonEvent) -> Result<(bool, UiEvent), String> {
        Ok((false, UiEvent::None))
    }

    fn on_drag(&mut self, _event: &GameMouseMotionEvent) -> Result<(bool, UiEvent), String> {
        Ok((false, UiEvent::None))
    }

    fn on_hover(&mut self, _event: &GameMouseButtonEvent) -> Result<(bool, UiEvent), String> {
        Ok((false, UiEvent::None))
    }
}
