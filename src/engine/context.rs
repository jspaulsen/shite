use crate::graphics::WindowContext;
use crate::world::GameWorld;

pub struct Context {
    pub window: WindowContext,
    pub world: GameWorld,
}

impl Context {
    pub fn new(window: WindowContext, world: GameWorld) -> Self {
        Self {
            window,
            world,
        }
    }
}
