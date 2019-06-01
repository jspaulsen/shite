use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::BlendMode;

use super::GameCanvas;

pub struct DimensionalRect;

impl DimensionalRect {
    pub fn render(canvas: &mut GameCanvas, rect: Rect, color: Color, border_width: Option<u32>, invert: bool) -> Result<(), String> {
        let dark_color = Color::RGBA(0, 0, 0, 100);
        let light_color = Color::RGBA(255, 255, 255, 100);
        let border = border_width.unwrap_or(2);

        let (upper_color, lower_color) = if invert { (dark_color, light_color) } else { (light_color, dark_color) };

        canvas.fill_rect(rect, color)?;

        canvas.draw(|canvas| {
            let (upper, left) = Self::upper_left(rect, border);
            let (lower, right) = Self::lower_right(rect, border);
            let rects = &[(upper, upper_color), (left, upper_color), (lower, lower_color), (right, lower_color)];

            canvas.canvas_mut().set_blend_mode(BlendMode::Blend);
            canvas.fill_rects(rects)
        })
    }

    fn upper_left(rect: Rect, border_width: u32) -> (Rect, Rect) {
        (
            Rect::new(rect.x(), rect.y() + border_width as i32, border_width, rect.height() - border_width),
            Rect::new(rect.x(), rect.y(), rect.width(), border_width),
        )
    }

    fn lower_right(rect: Rect, border_width: u32) -> (Rect, Rect) {
        (
            Rect::new(
                rect.x() + rect.width() as i32 - border_width as i32,
                rect.y() + border_width as i32,
                border_width,
                rect.height() - (2 * border_width),
            ),
            Rect::new(
                rect.x() + border_width as i32,
                rect.y() + rect.height() as i32 - border_width as i32,
                rect.width() - border_width,
                border_width,
            ),
        )
    }
}
