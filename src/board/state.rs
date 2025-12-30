use super::types::{CastlingRight, CastlingState, EnpassantState, NumOf, Side, Sides};

pub const NO_ENPASSANT: EnpassantState = NumOf::ENPASSANT_FILES as EnpassantState;
    
pub struct GameState {
    pub castling: CastlingState,
    pub enpassant: EnpassantState,
    pub side_to_move: Side,
}
impl GameState {
    pub fn new() -> Self {
        let castling_state = CastlingRight::WhiteKingSide as u8
            | CastlingRight::WhiteQueenSide as u8
            | CastlingRight::BlackKingSide as u8
            | CastlingRight::BlackQueenSide as u8;

        GameState {
            castling: castling_state,
            enpassant: NO_ENPASSANT,
            side_to_move: Sides::WHITE,
        }
    }
    pub fn revoke_right(&mut self, right: CastlingRight) {
        // We don't use XOR here because we want to clear the castling regardless of its previous state
        self.castling &= !(right as u8);
    }
    pub fn set_enpassant(&mut self, file_idx: EnpassantState) {
        debug_assert!(file_idx < NO_ENPASSANT);
        self.enpassant = file_idx;
    }

}
