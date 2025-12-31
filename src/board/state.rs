use super::{types::{CastlingRight, CastlingState, EnpassantState, NumOf, Sides}, zobrist::ZobristKey};
    
pub struct GameState {
    pub castling: CastlingState,
    pub enpassant: Option<EnpassantState>,
    pub active_color: u8,
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
            active_color: Sides::WHITE as u8,
            half_move_clock: 0,
            fullmove_counter: 0,
            zobrist_key: 0, 
        }
    }
    pub fn revoke_right(&mut self, right: CastlingRight) {
        // We don't use XOR here because we want to clear the castling regardless of its previous state
        self.castling &= !(right as u8);
    }
    pub fn set_enpassant(&mut self, file_idx: EnpassantState) {
        debug_assert!(file_idx < NumOf::ENPASSANT_FILES as EnpassantState);
        self.enpassant = Some(file_idx);
    }
    pub fn clear_enpassant(&mut self) {
        self.enpassant = None;
    }

}
