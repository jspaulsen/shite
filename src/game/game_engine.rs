use super::handler::{WindowHandler, EventHandler, StateHandler, BoxedGameState};

use sdl2;

/// This is the main GameEngine object which maintains all of the game state
/// and resources as well as provides the necessary logic for running the game.
pub struct GameEngine {
    pub event_handler: Box<EventHandler>,
    pub sdl_context: sdl2::Sdl,
    pub state_handler: StateHandler,
    pub window_handler: WindowHandler,
}

#[derive(Default)]
pub struct GameEngineBuilder {
    title: String,
    width: u32,
    height: u32,
    game_states: Vec<BoxedGameState>,
    active_state: String,
}


impl GameEngine {
    // TODO(#7): Implement GameEngine->run function
    // Run function runs the actual loop.  This will likely require
    // new() parameters to change as result & incorporate ticks_per_second
    // and (eventually) variable or fixed

    pub fn run(&mut self) -> Result<(), String> {
        loop {
            let active_state = self.state_handler.get_active_mut().ok_or("No active_state set!")?;
            self.event_handler.handle_sdl_events(active_state)?;
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
            height,
            game_states: Vec::new(),
            active_state: "".to_string(),
        }
    }

    pub fn set_active_game_state(mut self, name: String) -> Self {
        self.active_state = name;
        self
    }

    pub fn add_game_state(mut self, game_state: BoxedGameState) -> Self {
        self.game_states.push(game_state);
        self
    }

    pub fn add_multi_game_states(mut self, game_states: &mut Vec<BoxedGameState>) -> Self {
        self.game_states.append(game_states);
        self
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
            state_handler: StateHandler::new(self.active_state, self.game_states),
        })
    }
}
