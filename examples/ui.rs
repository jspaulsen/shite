extern crate sdl2;
extern crate shite;

use sdl2::keyboard::{
    KeyboardState,
    Keycode,
};
use sdl2::pixels::Color;
use sdl2::rect::Point;
use shite::engine::{
    Context,
    GameEngine,
    GameEngineBuilder,
};
use shite::event::{
    GameKeyboardEvent,
    UiEvent,
};
use shite::state::{
    GameApplicationHandler,
    GameInputHandler,
    GamePhysicsHandler,
    GameState,
    GameUiEventHandler,
    GameWindowHandler,
};
use shite::ui::{
    UiWindow,
    UiWindowBuilder,
};

pub struct UiState {
    ui: UiWindow,
    active: bool,
}

impl UiState {
    pub fn new(_ctx: &mut Context) -> Self {
        let builder = UiWindowBuilder::base(50, 50, 250, 250);
        let mut ui = builder.build();
        let _handle = ui.button(10, 10, 125, 25, None);

        Self {
            ui,
            active: false,
        }
    }
}

impl GameState for UiState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), String> {
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), String> {
        let (width, _) = context.window.canvas_mut().window().size();
        let canvas = context.window.canvas_mut();

        for y in 0..255 {
            canvas.set_draw_color(Color::RGB(y, y, y));
            canvas.draw_line(Point::new(0, i32::from(y)), Point::new(width as i32, i32::from(y)))?;
        }

        Ok(())
    }

    fn active_ui(&self) -> Option<&UiWindow> {
        if self.active {
            Some(&self.ui)
        } else {
            None
        }
    }

    fn active_ui_mut(&mut self) -> Option<&mut UiWindow> {
        if self.active {
            Some(&mut self.ui)
        } else {
            None
        }
    }
}

impl GameApplicationHandler for UiState {}
impl GameInputHandler for UiState {
    fn on_key_press(&mut self, _ctx: &mut Context, event: &GameKeyboardEvent, _kbd: &KeyboardState) -> Result<(), String> {
        if let Some(Keycode::Escape) = event.keycode {
            self.active = !self.active;
        }

        Ok(())
    }
}
impl GamePhysicsHandler for UiState {}
impl GameWindowHandler for UiState {}
impl GameUiEventHandler for UiState {
    fn on_ui_event(&mut self, _context: &mut Context, event: UiEvent) -> Result<(), String> {
        println!("UiEvent: {:?}", event);
        Ok(())
    }
}

fn main() {
    let mut engine: GameEngine = GameEngineBuilder::new("Ui", 800, 600, 60).build().unwrap();

    let mut context = engine.context_mut();
    let mut state = UiState::new(&mut context);

    engine.run(&mut state).unwrap();
}
