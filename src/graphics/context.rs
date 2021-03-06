use sdl2;

use super::game_canvas::GameCanvas;
use super::texture::GameTextureCreator;

/// WindowContext is _technically_ a misnomer; it provides an abstraction around
/// the Canvas<Window> class which handles the drawing (and is arguably more usefully
/// defined as a "Window").  The base Window class can be retrieved from the WindowCanvas.
pub struct WindowContext {
    canvas: sdl2::render::WindowCanvas,
}

pub struct WindowContextBuilder<'a> {
    subsystem: &'a sdl2::VideoSubsystem,
    title: &'a str,
    width: u32,
    height: u32,
}

impl<'a> WindowContextBuilder<'a> {
    pub fn new(subsystem: &'a sdl2::VideoSubsystem, title: &'a str, width: u32, height: u32) -> Self {
        Self {
            subsystem,
            title,
            width,
            height,
        }
    }

    pub fn build(self) -> Result<WindowContext, String> {
        let win_ref = self.subsystem.window(self.title, self.width, self.height).build().map_err(|e| e.to_string())?;

        Ok(WindowContext::new(win_ref.into_canvas().build().map_err(|e| e.to_string())?))
    }
}

impl WindowContext {
    pub fn new(canvas: sdl2::render::WindowCanvas) -> Self {
        Self {
            canvas,
        }
    }

    pub fn game_canvas(&mut self) -> GameCanvas {
        GameCanvas::new(&mut self.canvas)
    }

    pub fn window_canvas(&self) -> &sdl2::render::WindowCanvas {
        &self.canvas
    }

    pub fn canvas_mut(&mut self) -> &mut sdl2::render::WindowCanvas {
        &mut self.canvas
    }

    pub fn game_texture_creator(&self) -> GameTextureCreator {
        self.canvas.texture_creator()
    }
}
