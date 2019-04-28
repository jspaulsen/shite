use std::slice::Iter;

use crate::game::Renderable;

pub type BoxedRenderable = Box<Renderable>;

pub trait State {
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
    // TODO: update parameters will change
    fn update(&self) {
        let renderable = self.get_renderable();

        for render in renderable {
            if render.should_render() {
                /* Do some rendering */
            }
        }
    }
}
