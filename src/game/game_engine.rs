use super::graphics::window_handler;

use sdl2;

/// This is the main GameEngine object which maintains all of the game state
/// and resources as well as provides the necessary logic for running the game.
// TODO(#1): Abstract SDL (video) contexts
// SDL and related fields are likely not what we want to pass around;
// create an abstraction around them.
pub struct GameEngine {
    pub sdl_context: sdl2::Sdl,
    pub window_handler: window_handler::WindowHandler,
}

// TODO: Introduce builder pattern for GameEngine
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
    // TODO(#2): Window creation should be in abstraction layer
    // This should be a feature of the SDL abstraction layer and not arguments.
    // This is a dependency of ticket: `Abstract SDL (video) contexts`
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

    // TODO: Implement GameEngine->run function
    // Run function runs the actual loop.  This will likely require
    // new() parameters to change as result & incorporate ticks_per_second
    // and (eventually) variable or fixed
    pub fn run(&self) -> Result<(), String> {
        Ok(())
    }
}
