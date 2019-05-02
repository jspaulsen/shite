use sdl2::event::Event;

use crate::game::Renderable;


pub type BoxedRenderable = Box<Renderable>;
pub type BoxedGameState = Box<GameState>;


pub trait GameState: GameEventHandler {
    /// Returns the name of the GameState instance.  This is used for lookup
    /// and must be unique.
    ///
    /// Arguments:
    ///     None
    //
    /// Returns
    fn get_state_name(&self) -> String;
}
    /// Returns an iterator of Renderable objects
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// Iter<BoxedRenderable>
    //fn get_renderable(&self) -> Iter<BoxedRenderable>;

    /// Generically handles rendering objects within returned
    /// by the state object
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    // TODO(#4): update parameters will change
    /*
    fn update(&self) {
        let renderable = self.get_renderable();

        for render in renderable {
            if render.should_render() {
                /* Do some rendering */
            }
        }
    }
    */


pub trait GameEventHandler {
    fn on_key_down(&mut self, event: &Event);
    fn on_key_up(&mut self, event: &Event);
    fn on_mouse_motion(&mut self, event: &Event);
    fn on_mouse_button_down(&mut self, event: &Event);
    fn on_mouse_button_up(&mut self, event: &Event);
    fn on_mouse_wheel(&mut self, event: &Event);
    //fn on_window_event(&mut self, event: &Event, window: &Window);
}
