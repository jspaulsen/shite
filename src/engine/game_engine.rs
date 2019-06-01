use fps_clock::FpsClock;
use sdl2;

use super::context::Context;
use super::game_events::GameEvents;
use crate::graphics::{
    GameCanvas,
    WindowContext,
    WindowContextBuilder,
};
use crate::state::GameState;
use crate::ui::{
    Space,
    Ui,
};
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
    pub fn run(&mut self, game_state: &mut impl GameState) -> Result<(), String> {
        let mut fps = FpsClock::new(self.fps);

        loop {
            // TODO: SDL events are routed to both UI and GameState; this either
            // handles that (via the process_sdl_events function) or is broken
            // into two calls
            self.events.process_sdl_events(game_state, &mut self.context)?;

            // TODO(#21): Implement variable dt
            self.context.world.step();
            self.events.process_physics_events(game_state, &mut self.context)?;

            // update
            game_state.update(&mut self.context)?;

            // TODO: [GameEngine] move rendering from direct gameloop into function
            // Drop _all_ of the rendering into a separate function,
            // as it's clearly becoming a bit much to maintain in this loop.
            self.context.window.canvas_mut().clear();
            game_state.draw(&mut self.context)?;
            Self::render_state_ui(game_state, &mut self.context.window)?;
            self.context.window.canvas_mut().present();

            fps.tick();
        }
    }

    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    // TODO: [GameEngine] move render_state_ui elsewhere
    fn render_state_ui(game_state: &mut impl GameState, context: &mut WindowContext) -> Result<(), String> {
        let active_ui = game_state.active_ui();
        let creator = context.game_texture_creator();

        if let Some(ui) = active_ui {
            let rect = ui.space().rect();
            let mut texture = creator.create_texture_target(None, rect.width(), rect.height()).map_err(|e| e.to_string())?;
            let mut res: Result<(), String> = Ok(());

            context
                .canvas_mut()
                .with_texture_canvas(&mut texture, |canvas| {
                    res = ui.render(&mut GameCanvas::new(canvas), Some(Space::new(rect.x(), rect.y(), 0, 0, false)));
                })
                .map_err(|e| e.to_string())?;

            res?;
            context.canvas_mut().copy(&texture, None, rect)
        } else {
            Ok(())
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
        let window = WindowContextBuilder::new(&video, &self.title, self.width, self.height).build()?;
        let events = GameEvents::new(sdl_context.event_pump()?);

        Ok(GameEngine {
            events: Box::new(events),
            sdl_context,
            context: Context::new(window, self.world),
            fps: self.fps,
        })
    }
}
