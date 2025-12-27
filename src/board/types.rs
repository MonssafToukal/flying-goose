use super::bitboard;
use bitboard::BitBoard;

pub struct Board {
   bb_pieces: [BitBoard; 4],
   bb_sides: [BitBoard; Sides::Both as usize],
}

#[repr(usize)]
pub enum Sides {
    White,
    Black,
    Both,
}

enum GameState {
    
}

