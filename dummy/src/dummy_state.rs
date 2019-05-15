//use nphysics2d::object::ColliderHandle;
use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use shite::engine::Context;
use shite::graphics::GameTextureCreator;
use shite::state::{
    GameState,
    GameInputHandler,
    GamePhysicsHandler,
};

pub struct DummyState {
    pub x: i32,
    pub y: i32,
    pub should_render: bool,
    pub texture_creator: GameTextureCreator,
}

impl DummyState {
    pub fn new(x: i32, y: i32, should_render: bool, context: &Context) -> Self {
        Self {
            x,
            y,
            should_render,
            texture_creator: context.window.get_canvas_ref().texture_creator(),
        }
    }
}

impl GameInputHandler for DummyState {
    fn on_mouse_motion(&mut self, _context: &mut Context, event: &Event) -> Result<(), String> {
        match event {
            Event::MouseMotion { x, y, .. } => {
                self.x = *x;
                self.y = *y;
            },
            _ => {},
        };

        Ok(())
    }

    fn on_mouse_button_down(&mut self, _context: &mut Context, event: &Event) -> Result<(), String> {
        if let Event::MouseButtonDown{ mouse_btn, .. } = event {
            if let MouseButton::Left = mouse_btn {
                self.should_render = true;
            }
        }

        Ok(())
    }

    fn on_mouse_button_up(&mut self, _context: &mut Context, event: &Event) -> Result<(), String> {
        if let Event::MouseButtonDown{ mouse_btn, .. } = event {
            if let MouseButton::Left = mouse_btn {
                self.should_render = false;
            }
        }

        Ok(())
    }
}

impl GamePhysicsHandler for DummyState {}

impl GameState for DummyState {
    fn update(&mut self, _context: &mut Context) -> Result<(), String> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), String> {
        if self.should_render {
            let curr_color = context.window.get_canvas_ref().draw_color();
            let rect = Rect::new(self.x - 13, self.y - 13, 25, 25);

            context.window.get_canvas_mut().set_draw_color(
                Color::RGB(255, 255, 255)
            );
            context.window.get_canvas_mut().fill_rect(rect)?;

            context.window.get_canvas_mut().set_draw_color(curr_color);
        }

        Ok(())
    }
}
