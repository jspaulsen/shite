use fps_clock::FpsClock;
use sdl2;

use super::context::Context;
use super::game_events::GameEvents;
use crate::graphics::{WindowContextBuilder};
use crate::state::GameState;
use crate::world::GameWorld;


/// This is the main GameEngine object which maintains all of the game state
/// and resources as well as provides the necessary logic for running the game.
pub struct GameEngine {
    pub events: Box<GameEvents>,
    pub sdl_context: sdl2::Sdl,
    pub context: Context,
    fps: u32,
}

pub struct GameEngineBuilder {
    title: String,
    width: u32,
    height: u32,
    world: GameWorld,
    fps: u32,
}

impl GameEngine {

    pub fn run<T: GameState>(&mut self, game_state: &mut T) -> Result<(), String> {
        let mut fps = FpsClock::new(self.fps);

        loop {
            self.events.process_sdl_events(game_state, &mut self.context)?;

            // TODO(#21): Implement variable dt
            self.context.world.step();
            self.events.process_physics_events(game_state, &mut self.context)?;

            // update
            game_state.update(&mut self.context)?;

            self.context.window.get_canvas_mut().clear();
            game_state.draw(&mut self.context)?;
            self.context.window.get_canvas_mut().present();

            fps.tick();
            //println!("last_tick: {}", last_tick / 1_000_000.);
        }
    }

    pub fn get_context(&self) -> &Context {
        &self.context
    }

    pub fn get_context_mut(&mut self) -> &mut Context {
        &mut self.context
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
    pub fn new(title: &str, width: u32, height: u32, fps: u32) -> Self {
        Self {
            title: title.to_string(),
            width,
            height,
            world: GameWorld::new(),
            fps,
        }
    }

    /// Builds the GameEngine
    pub fn build(self) -> Result<GameEngine, String> {
        let sdl_context = sdl2::init()?;
        let video = sdl_context.video()?;
        let window = WindowContextBuilder::new(
            &video,
            &self.title,
            self.width,
            self.height,
        ).build()?;
        let events = GameEvents::new(
            sdl_context.event_pump()?,
        );


        Ok(GameEngine {
            events: Box::new(events),
            sdl_context,
            context: Context::new(window, self.world),
            fps: self.fps,
        })
    }
}
