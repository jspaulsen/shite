use super::window;
use super::window_builder::WindowBuilder;

use sdl2;

// TODO(#8): Support multiple windows
// Support multiple windows.  Currently only supports a single window.
pub struct WindowHandler {
    pub video_subsystem: sdl2::VideoSubsystem,
    main_window: window::Window,
}


// TODO: Expand WindowHandler to allow more options (if necessary)
// Expand to allow more options for main window (if necessary)
impl WindowHandler {
    pub fn new(
        video_subsystem: sdl2::VideoSubsystem,
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        let main_window = WindowBuilder::new(
            &video_subsystem,
            title,
            width,
            height,
        ).build()?;

        Ok(Self {
            video_subsystem,
            main_window,
        })
    }

    pub fn get_main_window(&self) -> &window::Window {
        &self.main_window
    }

    pub fn get_main_window_mut(&mut self) -> &mut window::Window {
        &mut self.main_window
    }
}
