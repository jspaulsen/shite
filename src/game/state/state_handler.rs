use super::game_state::GameState;

pub type BoxedGameState = Box<GameState>;

// TODO(#10): Implement StateHandler
struct StateHandler {
    active: Option<BoxedGameState>,
    states: Vec<BoxedGameState>,
}
