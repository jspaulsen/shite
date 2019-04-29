
use sdl2;


/// Window is _technically_ a misnomer; it provides an abstraction around
/// the Canvas<Window> class which handles the drawing (and is arguably more usefully
/// defined as a "Window").  The base Window class can be retrieved from the WindowCanvas.
pub struct Window {
    canvas_ref: sdl2::render::WindowCanvas,
}


impl Window {
    pub fn new(canvas_ref: sdl2::render::WindowCanvas) -> Self {
        Self { canvas_ref }
    }
    
    pub fn get_canvas_ref(&self) -> &sdl2::render::WindowCanvas {
        &self.canvas_ref
    }

    pub fn get_canvas_mut(&mut self) -> &mut sdl2::render::WindowCanvas {
        &mut self.canvas_ref
    }
}
