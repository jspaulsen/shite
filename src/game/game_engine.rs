//extern crate sdl2;

use sdl2;

/// This is the main GameEngine object which maintains all of the game state
/// and resources as well as provides the necessary logic for running the game.
// TODO: Abstract SDL (video) contexts
// SDL and related fields are likely not what we want to pass around;
// create an abstraction around them.
pub struct GameEngine {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
}

impl GameEngine {
    /// Returns an instance of the GameEngine object.
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// Result<Self, String>
    // TODO: Window creation should be in abstraction layer
    // This should be a feature of the SDL abstraction layer and not arguments.
    // This is a dependency of ticket: `Abstract SDL (video) contexts`
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;
        /*
        let window = video_subsystem.window("Experimental", 800, 600)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        loop {
            canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 255, 255));
            canvas.clear();
            canvas.present();
        };
        */

        Ok(Self {
            sdl_context,
            video_subsystem,
        })
    }
}
