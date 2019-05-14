mod box_state;
mod box_object;

use uuid::Uuid;
use shite::engine::{GameEngine, GameEngineBuilder};
use shite::state::{GameState};
use box_state::BoxState;


fn main() {
    let mut engine: GameEngine = GameEngineBuilder::new(
        "Boxes",
        800,
        600,
        30,
    ).build().unwrap();
    let mut context = engine.get_context_mut();
    let mut dummy_state = BoxState::new(&mut context);

    engine.run(&mut dummy_state).unwrap();
}
