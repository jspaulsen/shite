use sdl2::event::Event;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use shite::engine::Context;
use shite::graphics::GameTextureCreator;
use shite::state::{
    GameState,
    GameEventHandler,
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

impl GameEventHandler for DummyState {
    fn on_key_down(&mut self, context: &mut Context, event: &Event) {
        println!("on_key_down: {:?}", event);
        match event {
            Event::KeyDown { .. } => {
                println!("on_key_down: {:?}", event);
            }
            _ => {}
        };
    }

    fn on_key_up(&mut self, context: &mut Context, event: &Event)  {
        println!("on_key_up: {:?}", event);
    }

    fn on_mouse_motion(&mut self, context: &mut Context, event: &Event)  {
        match event {
            Event::MouseMotion { x, y, mousestate, .. } => {
                self.x = *x;
                self.y = *y;
            },
            _ => {},
        };
    }

    fn on_mouse_button_down(&mut self, context: &mut Context, event: &Event)  {
        if let Event::MouseButtonDown{ mouse_btn, .. } = event {
            if let MouseButton::Left = mouse_btn {
                self.should_render = true;
            }
        }
    }

    fn on_mouse_button_up(&mut self, context: &mut Context, event: &Event)  {
        if let Event::MouseButtonUp{ mouse_btn, .. } = event {
            if let MouseButton::Left = mouse_btn {
                self.should_render = false;
            }
        }
    }

    fn on_mouse_wheel(&mut self, context: &mut Context, event: &Event)  {
        println!("on_mouse_wheel: {:?}", event);
    }
}

impl GameState for DummyState {
    fn update(&mut self, context: &mut Context) -> Result<(), String> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), String> {
        if self.should_render {
            let curr_color = context.window.get_canvas_ref().draw_color();
            let rect = Rect::new(self.x - 13, self.y - 13, 25, 25);

            context.window.get_canvas_mut().set_draw_color(
                Color::RGB(255, 255, 255)
            );
            context.window.get_canvas_mut().draw_rect(rect);
            context.window.get_canvas_mut().fill_rect(rect);

            context.window.get_canvas_mut().set_draw_color(curr_color);
        }

        Ok(())
    }
}
