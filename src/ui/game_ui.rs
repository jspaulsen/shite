use sdl2::pixels::Color;

use super::Space;
use crate::graphics::GameCanvas;

pub type UiHandle = u64;

pub trait Ui {
    fn handle(&self) -> UiHandle;
    fn background_color(&self) -> Color;
    fn set_background_color(&mut self, color: Color);
    fn space(&self) -> &Space;

    fn render(&self, _canvas: &mut GameCanvas, _rel_to: Option<Space>) -> Result<(), String> {
        Ok(())
    }
}
