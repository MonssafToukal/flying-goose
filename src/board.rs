pub mod defs;
pub mod fen;
pub mod history;
pub mod initialization;
pub mod piece_movement;
pub mod state;
pub mod types;
pub mod zobrist;

use crate::board::types::{NumOf, Piece, Sides};
use history::GameHistory;
use state::GameState;
use types::BitBoard;
use zobrist::Zobrist;

#[derive(Debug, Clone)]
pub struct Board {
    pub bb_pieces: [[BitBoard; NumOf::PIECE_TYPES]; Sides::BOTH],
    pub bb_sides: [BitBoard; Sides::BOTH],
    pub piece_list: [Piece; NumOf::SQUARES],
    pub game_state: GameState,
    pub history: GameHistory,
    pub zobrist_hashmap: Zobrist,
}
