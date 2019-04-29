use super::window;

use sdl2;


pub struct WindowBuilder<'a> {
    subsystem: &'a sdl2::VideoSubsystem,
    title: &'a str,
    width: u32,
    height: u32,
}


impl <'a>WindowBuilder<'a> {
    pub fn new(
        subsystem: &'a sdl2::VideoSubsystem,
        title: &'a str,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            subsystem,
            title,
            width,
            height,
        }
    }

    pub fn build(self) -> Result<window::Window, String> {
        let win_ref = self.subsystem.window(
            self.title,
            self.width,
            self.height,
        ).build().map_err(|e| e.to_string())?;

        Ok(window::Window::new(
            win_ref.into_canvas()
                .build()
                .map_err(|e| e.to_string())?)
        )
    }
}
