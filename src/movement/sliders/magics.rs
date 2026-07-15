use rand::Rng;
use rand_pcg::Pcg64;
use std::fmt::Display;

use crate::{
    board::types::{EMPTY_BITBOARD, FULL_BITBOARD},
    types::{BitBoard, NumOf},
};

pub const MAX_ROOK_TABLE_SIZE: usize = 102400;
pub const MAX_BISHOP_TABLE_SIZE: usize = 5248;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagicEntry {
    pub number: u64,
    pub blocker_mask: BitBoard,
    pub inverse_blocker_mask: BitBoard,
    pub offset: u32,
    pub index_bits: u8,
    pub shift: u8,
}

impl Display for MagicEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Magic number: {} offset: {} index_bits: {} shift: {}",
            self.number, self.offset, self.index_bits, self.shift
        )
    }
}

impl Default for MagicEntry {
    fn default() -> Self {
        MagicEntry {
            number: 0,
            blocker_mask: EMPTY_BITBOARD,
            inverse_blocker_mask: FULL_BITBOARD,
            offset: NumOf::SQUARES as u32,
            index_bits: 0,
            shift: NumOf::SQUARES as u8,
        }
    }
}

impl MagicEntry {
    pub fn new(rng: &mut Pcg64, blocker_mask: BitBoard) -> Self {
        let mut number_of_bits_set = 0;
        let mut temp_blocker = blocker_mask;
        while temp_blocker != 0 {
            number_of_bits_set += 1;
            temp_blocker &= temp_blocker - 1;
        }
        let mut magic_numbers = [0u64; 3];
        rng.fill(&mut magic_numbers);
        let magic_number: u64 = magic_numbers.into_iter().reduce(|acc, m| acc & m).unwrap();
        let num_squares: u8 = u8::try_from(NumOf::SQUARES).unwrap();
        Self {
            number: magic_number,
            blocker_mask: blocker_mask,
            inverse_blocker_mask: !blocker_mask,
            offset: 0,
            index_bits: number_of_bits_set,
            shift: num_squares - number_of_bits_set,
        }
    }
}
