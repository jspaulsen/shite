extern crate nalgebra as na;

use na::Vector2;
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use nphysics2d::object::{
    BodyHandle,
    ColliderDesc,
    RigidBodyDesc,
};
use sdl2::pixels::Color;
use shite::trackable::Trackable;
use shite::world::GameWorld;
use uuid::Uuid;


#[derive(Debug)]
pub struct BoxObject {
    pub body_handle: BodyHandle,
    pub color: Color,
    pub handle: Uuid,
    pub width: u32,
    pub height: u32,
}

pub struct BoxObjectBuilder {
    pub x: i32,
    pub y: i32,
    pub color: Color,
    pub width: u32,
    pub height: u32,
    pub velocity: Vector2<f64>
}

impl BoxObjectBuilder {
    pub fn new(width: u32, height: u32, x: i32, y: i32, color: Color, velocity: Vector2<f64>) -> Self {
        Self {
            x,
            y,
            color,
            width,
            height,
            velocity,
        }
    }

    pub fn build(self, world: &mut GameWorld) -> BoxObject {
        let cube = ShapeHandle::new(Cuboid::new(Vector2::new(self.width as f64 / 2.0, self.height as f64 / 2.0)));
        let collider_desc = ColliderDesc::new(cube).material(MaterialHandle::new(BasicMaterial::new(0.8, 0.3)));
        let body = RigidBodyDesc::new()
            .mass(1.5)
            .collider(&collider_desc)
            .velocity(Velocity2::new(self.velocity, 0.0))
            .translation(Vector2::new(self.x as f64, self.y as f64))
            .build(world);

        BoxObject {
            body_handle: body.handle(),
            color: self.color,
            handle: Uuid::new_v4(),
            width: self.width,
            height: self.height,
        }
    }
}

impl BoxObject {
    pub fn get_body_handle(&self) -> &BodyHandle {
        &self.body_handle
    }
}

impl Trackable for BoxObject {
    fn get_handle(&self) -> &Uuid {
        &self.handle
    }
}
