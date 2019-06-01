use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

pub struct GameCanvas<'a> {
    canvas: &'a mut WindowCanvas,
}

impl<'a> GameCanvas<'a> {
    pub fn new(canvas: &'a mut WindowCanvas) -> Self {
        GameCanvas {
            canvas,
        }
    }

    pub fn canvas(&self) -> &WindowCanvas {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut WindowCanvas {
        &mut self.canvas
    }

    pub fn fill_rect(&mut self, rect: Rect, color: Color) -> Result<(), String> {
        self.draw(|canvas| {
            canvas.canvas.set_draw_color(color);
            canvas.canvas.fill_rect(rect)
        })
    }

    pub fn fill_rects(&mut self, rects: &[(Rect, Color)]) -> Result<(), String> {
        for (r, c) in rects.iter() {
            self.fill_rect(*r, *c)?;
        }

        Ok(())
    }

    pub fn draw<F>(&mut self, mut f: F) -> Result<(), String>
    where
        F: FnMut(&mut GameCanvas) -> Result<(), String>,
    {
        let curr_blend = self.canvas.blend_mode();
        let curr_color = self.canvas.draw_color();

        let res = f(self);

        self.canvas.set_blend_mode(curr_blend);
        self.canvas.set_draw_color(curr_color);

        res
    }

    pub fn renderer<F>(&mut self, mut f: F) -> Result<(), String>
    where
        F: FnMut(&mut GameCanvas) -> Result<(), String>,
    {
        self.canvas.clear();
        let res = f(self);
        self.canvas.present();

        res
    }
}
