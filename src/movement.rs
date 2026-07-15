pub mod sliders;
use sliders::magics::{MAX_BISHOP_TABLE_SIZE, MAX_ROOK_TABLE_SIZE};

use crate::{board::types::EMPTY_BITBOARD, types::BitBoard};

pub struct MovementData {
    pub rook_lut: Vec<BitBoard>,
    pub bishop_lut: Vec<BitBoard>,
}

impl MovementData {
    pub fn new() -> Self {
        Self {
            rook_lut: vec![EMPTY_BITBOARD; MAX_ROOK_TABLE_SIZE],
            bishop_lut: vec![EMPTY_BITBOARD; MAX_BISHOP_TABLE_SIZE],
        }
    }
}
