use super::handler::window_handler;

use sdl2;

/// This is the main GameEngine object which maintains all of the game state
/// and resources as well as provides the necessary logic for running the game.
pub struct GameEngine {
    pub sdl_context: sdl2::Sdl,
    pub window_handler: window_handler::WindowHandler,
}

// TODO(#6): Introduce builder pattern for GameEngine
impl GameEngine {
    /// Returns an instance of the GameEngine object.
    ///
    /// # Arguments
    /// * title - Window Title
    /// * width - Window width
    /// * height - Window height
    ///
    /// # Returns
    /// Result<Self, String>
    pub fn new(
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let window_handler = window_handler::WindowHandler::new(
            sdl_context.video()?,
            title,
            width,
            height,
        )?;

        Ok(Self {
            sdl_context,
            window_handler,
        })
    }

    // TODO(#7): Implement GameEngine->run function
    // Run function runs the actual loop.  This will likely require
    // new() parameters to change as result & incorporate ticks_per_second
    // and (eventually) variable or fixed
    pub fn run(&self) -> Result<(), String> {
        Ok(())
    }
}
