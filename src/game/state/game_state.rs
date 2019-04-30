use std::slice::Iter;

use crate::game::Renderable;


pub type BoxedRenderable = Box<Renderable>;


pub trait GameState {
    /// Returns the name of the GameState instance.  This is used for lookup
    /// and must be unique.
    ///
    /// Arguments:
    ///     None
    //
    /// Returns
    fn get_state_name(&self) -> String;

    /// Returns an iterator of Renderable objects
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// Iter<BoxedRenderable>
    fn get_renderable(&self) -> Iter<BoxedRenderable>;

    /// Generically handles rendering objects within returned
    /// by the state object
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// None
    // TODO(#4): update parameters will change
    fn update(&self) {
        let renderable = self.get_renderable();

        for render in renderable {
            if render.should_render() {
                /* Do some rendering */
            }
        }
    }
}
