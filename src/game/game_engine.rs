use super::handler::window_handler;

use sdl2;

/// This is the main GameEngine object which maintains all of the game state
/// and resources as well as provides the necessary logic for running the game.
pub struct GameEngine {
    pub sdl_context: sdl2::Sdl,
    pub window_handler: window_handler::WindowHandler,
}

pub struct GameEngineBuilder {
    title: String,
    width: u32,
    height: u32,
}


impl GameEngine {
    // TODO(#7): Implement GameEngine->run function
    // Run function runs the actual loop.  This will likely require
    // new() parameters to change as result & incorporate ticks_per_second
    // and (eventually) variable or fixed
    pub fn run(&self) -> Result<(), String> {
        Ok(())
    }
}


impl GameEngineBuilder {
    /// Returns an instance of the GameEngineBuilder object.
    ///
    /// # Arguments
    /// * title - Title of window when built
    /// * width - Width of window when built
    /// * height - Height of window when built
    ///
    /// # Returns
    /// Result<Self, String>
    pub fn new(title: String, width: u32, height: u32) -> Self {
        Self {
            title,
            width,
            height
        }
    }

    /// Builds the GameEngine
    pub fn build(self) -> Result<GameEngine, String> {
        let sdl_context = sdl2::init()?;
        let window_handler = window_handler::WindowHandler::new(
            sdl_context.video()?,
            &self.title,
            self.width,
            self.height,
        )?;

        Ok(GameEngine {
            sdl_context,
            window_handler,
        })
    }
}
