use super::{state::GameState, types::MAX_GAME_MOVES};

pub struct GameHistory {
    moves: [GameState; MAX_GAME_MOVES as usize],
    count: usize,
}

impl GameHistory {
    pub fn new() -> Self {
        Self {
            moves: [GameState::new(); MAX_GAME_MOVES as usize],
            count: 0,
        } 
    }
    pub fn pop(&mut self) -> Option<GameState> {
        if self.count > 0 {
            self.count -= 1;
            Some(self.moves[self.count])
        } else {
            None
        }
    }
    pub fn push(&mut self, game_state: GameState) {
        self.moves[self.count]  = game_state;
        self.count += 1;
    }
    pub fn get(&self, index: usize) -> &GameState {
        debug_assert!(index < self.count);
        &self.moves[index]
    }
    pub fn len(&self) -> usize {
        self.count
    }
    pub fn clear(&mut self) {
       self.count = 0;
    }
}

impl Default for GameHistory {
    fn default() -> Self {
        Self::new()
    }
}
