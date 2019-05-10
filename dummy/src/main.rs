mod dummy_state;

use shite::engine::{GameEngine, GameEngineBuilder};
use dummy_state::DummyState;


fn main() {
    let mut engine: GameEngine = GameEngineBuilder::new(
        "Dummy",
        800,
        600,
        60,
    ).build().unwrap();
    let context = engine.get_context();
    let mut dummy_state = DummyState::new(0, 0, false, context);

    engine.run(&mut dummy_state).unwrap();
}
