use nphysics2d::world::World;
use nphysics2d::object::{ColliderDesc, RigidBodyDesc};

pub type N = f64;
pub type GameWorld = World<N>;
pub type GameRigidBodyDesc<'a> = RigidBodyDesc<'a, N>;
pub type GameColliderDesc = ColliderDesc<N>;
