use nphysics2d::object::ColliderHandle;
use nphysics2d::world::World;

pub type N = f64;
pub type GameWorld = World<N>;

#[derive(Debug)]
pub enum GameCollisionEvent {
    CollisionStart(ColliderHandle, ColliderHandle),
    CollisionEnd(ColliderHandle, ColliderHandle),
}
