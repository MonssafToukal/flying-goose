use crate::{board::types::Square, types::{BitBoard, EMPTY_BITBOARD, FILE_MASKS, NumOf, RANK_MASKS, SQUARE_MASKS}};

#[repr(u8)]
pub enum KingDirections {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl KingDirections {
    pub const NORTH_SHIFT: i8 = 8;
    pub const SOUTH_SHIFT: i8 = -8;
    pub const EAST_SHIFT: i8 = 1;
    pub const WEST_SHIFT: i8 = -1;

    pub fn shift(&self, bb: BitBoard) -> BitBoard {
        let _NORTH_SHIFT = 8;
        let _SOUTH_SHIFT = 8;
        let _EAST_SHIFT = 1;
        let _WEST_SHIFT = 1;
        match self {
            KingDirections::NORTH => bb << _NORTH_SHIFT,
            KingDirections::SOUTH => bb >> _SOUTH_SHIFT,
            KingDirections::EAST => bb << _EAST_SHIFT,
            KingDirections::WEST => bb >> _WEST_SHIFT,
        }
    }
}

// The first cardinal direction mentioned is the one that takes 2 squares
// e.g SouthEast => 2 squares down and one square right
#[derive(Debug)]
pub struct KnightDirections;
impl KnightDirections {
    pub const NORTH_EAST: i8 = 17;
    pub const NORTH_WEST: i8 = 15;
    pub const SOUTH_EAST: i8 = -15;
    pub const SOUTH_WEST: i8 = -17;
    pub const EAST_NORTH: i8 = 10;
    pub const EAST_SOUTH: i8 = -6;
    pub const WEST_NORTH: i8 = 6;
    pub const WEST_SOUTH: i8 = -10;
}

pub fn get_king_attacks(king_square_idx: Square) -> BitBoard {
    const NOT_RANK_8: BitBoard = !RANK_MASKS[NumOf::RANKS - 1];
    const NOT_RANK_1: BitBoard = !RANK_MASKS[0];
    let king_position = SQUARE_MASKS[king_square_idx];
    let mut king_attacks = get_king_attacks_east_west(king_position);

    // North direction:
    king_attacks |= KingDirections::NORTH.shift(king_position & NOT_RANK_8);

    // South direction:
    king_attacks |= KingDirections::SOUTH.shift(king_position & NOT_RANK_1);

    king_attacks
}

#[inline(always)]
fn get_king_attacks_east_west(king_position: BitBoard) -> BitBoard {
    const NOT_FILE_H: BitBoard = !FILE_MASKS[NumOf::FILES - 1];
    const NOT_FILE_A: BitBoard = !FILE_MASKS[0];
    let mut attacks = EMPTY_BITBOARD;
    // East direction:
    attacks |= KingDirections::EAST.shift(king_position & NOT_FILE_H);
    // West direction:
    attacks |= KingDirections::WEST.shift(king_position & NOT_FILE_A);

    attacks
}
