use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use super::GameCanvas;
const DEFAULT_OPAC: u8 = 25;

pub fn drop_shadow(canvas: &mut GameCanvas, rect: Rect, opacity: Option<u8>, shadow: Option<Color>) -> Result<(), String> {
    let opac = opacity.unwrap_or(DEFAULT_OPAC);
    let color = shadow.unwrap_or_else(|| Color::RGBA(0, 0, 0, opac));

    canvas.draw(|canvas| {
        canvas.canvas_mut().set_blend_mode(BlendMode::Blend);
        canvas.fill_rect(rect, color)
    })
}
