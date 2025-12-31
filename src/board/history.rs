use super::types::MAX_GAME_MOVES;

pub struct GameHistory {
    moves: [GameState; MAX_GAME_MOVES],
    count: usize,
}
