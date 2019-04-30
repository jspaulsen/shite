use std::collections::HashMap;

use super::game_state::GameState;


pub type BoxedGameState = Box<GameState>;

pub struct StateHandler {
    // TODO(#13): [StateHandler] This could be a bottleneck
    // Using a HashMap to lookup & maintain states is likely a bottleneck.
    active: String,
    game_states: HashMap<String, BoxedGameState>,
}

impl StateHandler {
    /// Creates a new instance of StateHandler and takes ownership of the provided values.
    ///
    /// # Arguments:
    /// * `active`: String, The name of the active GameState
    /// * `states`: Vec<BoxedGameState>, A list of initial GameStates.
    pub fn new(
        active: String,
        game_states: Vec<BoxedGameState>,
    ) -> Self {
        StateHandler {
            active,
            game_states: game_states.into_iter()
                .map(|x| (x.get_state_name(), x))
                .collect::<Vec<(_,_)>>()
                .into_iter()
                .collect(),
        }
    }

    pub fn get_active(&self) -> Option<&BoxedGameState> {
        self.game_states.get(&self.active)
    }

    pub fn get_active_mut(&mut self) -> Option<&mut BoxedGameState> {
        self.game_states.get_mut(&self.active)
    }

    /// Sets the active state to specified name if it exists.
    ///
    /// # Arguments
    /// * `name`, &str: Name to transition state to
    ///
    /// # Returns
    /// None
    ///
    // TODO: [StateHandler] Implement actual state transition
    // Transitioning between states requires "thoughtful" design
    // and an actual implementation
    pub fn set_active(&mut self, name: &str) {
        if self.game_states.contains_key(name) {
            self.active = name.to_owned();
        }
    }

    /// Adds a GameState to the StateHandler.  If an existing GameState exists with the same name
    /// it is replaced and returned.
    ///
    /// # Arguments:
    /// * `game_state`, BoxedGameState: GameState to add
    ///
    /// # Returns:
    /// Option<BoxedGameState>
    pub fn add_game_state(&mut self, game_state: BoxedGameState) -> Option<BoxedGameState> {
        self.game_states.insert(game_state.get_state_name(), game_state)
    }

    /// Removes a GameState by it's name and returns it if the game_name to remove is not active.
    ///
    /// # Arguments:
    /// * `name`: &str, name of GameState to remove
    ///
    /// # Returns:
    /// Option<BoxedGameState>
    ///
    /// # Remarks
    /// The GameState cannot be removed if it is active; another game state will need to be made
    /// active before removing the current.
    pub fn remove_game_state(&mut self, name: &str) -> Option<BoxedGameState> {
        if self.active == name {
            return None;
        }

        self.game_states.remove(name)
    }

}
