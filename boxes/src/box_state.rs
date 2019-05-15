extern crate nalgebra as na;

use na::Vector2;
use nphysics2d::algebra::Velocity2;
use nphysics2d::material::{MaterialHandle, BasicMaterial};
use ncollide2d::shape::{Cuboid, ShapeHandle};
use nphysics2d::object::{
    ColliderHandle,
    ColliderDesc,
};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Rect};

use shite::engine::Context;
use shite::state::{
    GameState,
    GameInputHandler,
    GamePhysicsHandler,
    PhysicsObjectMap,
};

use crate::box_object::{BoxObject, BoxObjectBuilder};


pub struct BoxState {
    pub game_objects: PhysicsObjectMap<BoxObject>,
}

impl BoxState {
    pub fn new(context: &mut Context) -> Self {
        let (x, y) = context.window.get_canvas_ref().window().size();
        let ground_shape = ShapeHandle::new(Cuboid::new(Vector2::new(x as f64, 1.0)));
        let wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(1.2, y as f64)));

        let ground_desc = ColliderDesc::new(ground_shape)
            .material(MaterialHandle::new(BasicMaterial::new(1.2, 0.3)))
            .translation(Vector2::new(0.0, y as f64));
        let wall_desc = ColliderDesc::new(wall_shape)
            .material(MaterialHandle::new(BasicMaterial::new(1.2, 0.3)))
            .translation(Vector2::new(0.0, 0.0));

        context.world.set_gravity(
            Vector2::new(0.0, 9.81),//9.81),
        );

        ground_desc.build(&mut context.world);
        ground_desc.translation(Vector2::new(0.0, 0.0)).build(&mut context.world);

        wall_desc.build(&mut context.world);
        wall_desc.translation(Vector2::new(x as f64, 0.0)).build(&mut context.world);

        Self {
            game_objects: PhysicsObjectMap::new(),
        }
    }
}

impl GameInputHandler for BoxState {
    fn on_key_down(&mut self, context: &mut Context, event: &Event) -> Result<(), String> {
        if let Event::KeyDown { keycode: Some(Keycode::A), .. } = event {
            for (body, _) in self.game_objects.get_objects_mut() {
                let rigid_body = context.world.rigid_body_mut(*body).expect("Unwrapping body from nphysics world.");
                let veloc = rigid_body.velocity();
                let add_x = if veloc.linear.x > 0.0 { 100.0 } else { -100.0 };
                let add_y = if veloc.linear.y > 0.0 { 100.0 } else { -100.0 };

                rigid_body.set_velocity(
                    Velocity2::new(
                        Vector2::new(veloc.linear.x + add_x, veloc.linear.y + add_y),
                        veloc.angular,
                    ),
                );
            }
        }

        Ok(())
    }

    fn on_mouse_button_down(&mut self, context: &mut Context, event: &Event) -> Result<(), String> {
        if let Event::MouseButtonDown{ mouse_btn, x, y, .. } = event {
            if let MouseButton::Left = mouse_btn {
                let (win_x, win_y) = context.window.get_canvas_ref().window().size();
                let const_veloc = 125.0;
                let half_x: u32 = win_x / 2;
                let half_y: u32 = win_y / 2;

                let veloc_x = if *x as u32 > half_x { -1.0 * const_veloc } else { const_veloc };
                let veloc_y = if *y as u32 > half_y { -1.0 * const_veloc } else { const_veloc };
                let builder = BoxObjectBuilder::new(
                    25,
                    25,
                    *x,
                    *y,
                    Color::RGB(255, 255, 255),
                    Vector2::new(veloc_x, veloc_y),
                );
                let object = builder.build(&mut context.world);

                self.game_objects.add(*object.get_body_handle(), object);
            }
        }

        Ok(())
    }
}

impl GamePhysicsHandler for BoxState {
    fn on_collision_end(&mut self, context: &mut Context, coh1: ColliderHandle, coh2: ColliderHandle) -> Result<(), String> {
        let _coh_obj1 = context.world.collider(coh1).expect("Expected nphysics to provide a collider object");
        let _coh_obj2 = context.world.collider(coh2).expect("Expected nphysics to provide a collider object");

        Ok(())
    }
}

impl GameState for BoxState {
    fn update(&mut self, _context: &mut Context) -> Result<(), String> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), String> {
        let curr_color = context.window.get_canvas_ref().draw_color();

        for (_, object) in self.game_objects.get_objects() {
            let body = context.world.rigid_body(*object.get_body_handle());

            if let Some(rigid_body) = body {
                let vector = rigid_body.position().translation.vector;

                context.window.get_canvas_mut().set_draw_color(object.color);

                let rect = Rect::new(
                    (vector.x - object.width as f64) as i32,
                    (vector.y - object.height as f64) as i32,
                    object.width,
                    object.height,
                );

                context.window.get_canvas_mut().draw_rect(rect)?;
            }
        }
        context.window.get_canvas_mut().set_draw_color(curr_color);
        Ok(())
    }
}
