use super::game_state::GameState;

pub type BoxedGameState = Box<GameState>;

// TODO: Implement StateHandler
struct StateHandler {
    active: Option<BoxedGameState>,
    states: Vec<BoxedGameState>,
}
