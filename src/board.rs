pub mod defs;
pub mod fen;
pub mod helpers;
pub mod history;
pub mod initialization;
pub mod piece_movement;
pub mod state;
pub mod types;
pub mod zobrist;

use crate::{
    board::types::Piece,
    types::{BitBoard, NumOf},
};
use history::GameHistory;
use state::GameState;
use types::{BySide, BySquare};
use zobrist::Zobrist;

#[derive(Debug, Clone)]
pub struct Board {
    pub bb_pieces: BySide<[BitBoard; NumOf::PIECE_TYPES]>,
    pub bb_sides: BySide<BitBoard>,
    pub piece_list: BySquare<Piece>,
    pub game_state: GameState,
    pub history: GameHistory,
    pub zobrist_hashmap: Zobrist,
}
