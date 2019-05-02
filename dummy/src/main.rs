mod dummy_state;

use std::slice::Iter;

use sdl2::pixels::Color;
use shite::game::{GameEngine, GameEngineBuilder};

use dummy_state::DummyState;


fn main() {
    let dummy_state: DummyState = DummyState { };
    let mut engine: GameEngine = GameEngineBuilder::new(
        "Poopster",
        800,
        600,
    ).add_game_state(Box::new(dummy_state))
    .set_active_game_state("DummyState".to_string())
    .build()
    .unwrap();

    engine.run();
}

/*
fn on_key_down(&mut self, event: &Event);
fn on_key_up(&mut self, event: &Event);
fn on_mouse_motion(&mut self, event: &Event);
fn on_mouse_button_down(&mut self, event: &Event);
fn on_mouse_button_up(&mut self, event: &Event);
fn on_mouse_wheel(&mut self, event: &Event);
*/
