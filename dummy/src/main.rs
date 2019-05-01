use std::slice::Iter;

use sdl2::pixels::Color;

use shite::game::{GameEngine, GameEngineBuilder};

fn main() {
    let mut engine: GameEngine = GameEngineBuilder::new(
        "Poopster",
        800,
        600,
    ).build().unwrap();

    engine.run();
}
