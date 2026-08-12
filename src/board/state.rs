use crate::board::Piece;
use crate::board::types::CastlingRight;
use crate::board::types::{CastlingState, Sides, Square};

use crate::board::{types::Side, zobrist::ZobristKey};

#[derive(Clone, Copy, Debug)]
pub struct GameState {
    pub castling: CastlingState,
    pub captured_piece: Option<Piece>,
    pub enpassant: Option<Square>,
    pub active_color: Side,
    pub half_move_clock: u8,
    pub fullmove_counter: u16,
    pub zobrist_key: ZobristKey,
}
impl GameState {
    pub fn new() -> Self {
        let castling_state = CastlingRight::WhiteKingSide as u8
            | CastlingRight::WhiteQueenSide as u8
            | CastlingRight::BlackKingSide as u8
            | CastlingRight::BlackQueenSide as u8;

        GameState {
            castling: castling_state,
            enpassant: None,
            captured_piece: None,
            active_color: Sides::WHITE,
            half_move_clock: 0,
            fullmove_counter: 1,
            zobrist_key: 0,
        }
    }
    pub fn revoke_right(&mut self, right: CastlingRight) {
        // We don't use XOR here because we want to clear the castling regardless of its previous state
        self.castling &= !(right as u8);
    }
    pub fn set_enpassant(&mut self, square: Square) {
        self.enpassant = Some(square);
    }
    pub fn clear_enpassant(&mut self) {
        self.enpassant = None;
    }

    #[inline]
    pub fn has_right(&self, right: CastlingRight) -> bool {
        self.castling & (right as u8) != 0
    }

    #[inline]
    pub fn toggle_side(&mut self) {
        self.active_color ^= 1;
    }
}
