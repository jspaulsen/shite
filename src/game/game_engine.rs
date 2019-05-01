use super::handler::{WindowHandler, EventHandler};

use sdl2;

/// This is the main GameEngine object which maintains all of the game state
/// and resources as well as provides the necessary logic for running the game.
pub struct GameEngine {
    pub event_handler: Box<EventHandler>,
    pub sdl_context: sdl2::Sdl,
    pub window_handler: WindowHandler,
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

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            self.event_handler.handle_sdl_events()?;
        }
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
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height
        }
    }

    /// Builds the GameEngine
    pub fn build(self) -> Result<GameEngine, String> {
        let sdl_context = sdl2::init()?;
        let window_handler = WindowHandler::new(
            sdl_context.video()?,
            &self.title,
            self.width,
            self.height,
        )?;
        let event_handler = EventHandler::new(
            sdl_context.event()?,
            sdl_context.event_pump()?,
        );

        Ok(GameEngine {
            sdl_context,
            window_handler,
            event_handler: Box::new(event_handler),
        })
    }
}
