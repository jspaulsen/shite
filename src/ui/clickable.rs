use sdl2::event::Event;

use crate::engine::Context;
use crate::graphics::GameCanvas;

pub trait Clickable {
    fn on_click(&mut self, _ctx: &mut Context, _evnt: &Event) -> Result<(), String> {
        Ok(())
    }

    fn on_click_release(&mut self, _ctx: &mut Context, _evnt: &Event) -> Result<(), String> {
        Ok(())
    }
}

pub trait RenderableClick {
    fn render_clicked(&self, canvas: &mut GameCanvas) -> Result<(), String>;
    fn render_unclicked(&self, canvas: &mut GameCanvas) -> Result<(), String>;
}
