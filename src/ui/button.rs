use sdl2::pixels::Color;

use super::{
    Interfaceable,
    Space,
    Ui,
    UiHandle,
    Widget,
};
use crate::event::{
    GameMouseButtonEvent,
    UiEvent,
};
use crate::graphics::{
    DimensionalRect,
    GameCanvas,
};

#[derive(Debug)]
pub struct Button {
    space: Space,
    bg_color: Color,
    is_clicked: bool,
    handle: UiHandle,
}

impl Button {
    pub fn new(handle: UiHandle, space: Space, bg_color: Option<Color>) -> Self {
        let bg_color = bg_color.unwrap_or_else(|| Color::RGB(240, 248, 255));

        Self {
            space,
            bg_color,
            is_clicked: false,
            handle,
        }
    }
}

impl Interfaceable for Button {
    fn on_click(&mut self, _event: &GameMouseButtonEvent) -> Result<(bool, UiEvent), String> {
        self.is_clicked = true;

        Ok((self.is_clicked, UiEvent::Click(self.handle)))
    }

    fn on_click_release(&mut self, _event: &GameMouseButtonEvent) -> Result<(bool, UiEvent), String> {
        self.is_clicked = false;
        
        Ok((self.is_clicked, UiEvent::ClickRelease(self.handle)))
    }
}

impl Ui for Button {
    fn handle(&self) -> UiHandle {
        self.handle
    }

    fn background_color(&self) -> Color {
        self.bg_color
    }

    fn set_background_color(&mut self, color: Color) {
        self.bg_color = color
    }

    fn space(&self) -> &Space {
        &self.space
    }

    fn render(&self, canvas: &mut GameCanvas, rel_to: Option<Space>) -> Result<(), String> {
        let space = if let Some(space) = rel_to {
            space.relative_space(&self.space)
        } else {
            self.space.clone()
        };

        DimensionalRect::render(canvas, space.rect(), self.bg_color, None, self.is_clicked)
    }
}

impl Widget for Button {}
