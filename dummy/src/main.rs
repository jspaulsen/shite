mod dummy_state;

use uuid::Uuid;
use shite::engine::{GameEngine, GameEngineBuilder};
use shite::state::{GameState};
use dummy_state::DummyState;


fn main() {
    let mut engine: GameEngine = GameEngineBuilder::new(
        "Poopster",
        800,
        600,
    ).build().unwrap();
    let context = engine.get_context();
    let mut dummy_state = DummyState::new(0, 0, false, context);

    engine.run(&mut dummy_state);
}
