use crate::{
    board::types::{Files, Square},
    types::{BitBoard, EMPTY_BITBOARD, FILE_MASKS, NumOf, RANK_MASKS, SQUARE_MASKS},
};

#[repr(u8)]
pub enum KingDirections {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

impl KingDirections {
    // Leaving these for reference, this is the amount of bit shifting required for each direction
    // pub const _NORTH_SHIFT: i8 = 8;
    // pub const _SOUTH_SHIFT: i8 = -8;
    // pub const _EAST_SHIFT: i8 = 1;
    // pub const _WEST_SHIFT: i8 = -1;

    fn shift(&self, bb: BitBoard) -> BitBoard {
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
pub enum KnightDirections {
    NORTH_EAST,
    NORTH_WEST,
    SOUTH_EAST,
    SOUTH_WEST,
    EAST_NORTH,
    EAST_SOUTH,
    WEST_NORTH,
    WEST_SOUTH,
}

impl KnightDirections {
    // Leaving these for reference, this is the amount of bit shifting required for each direction
    // const _NORTH_EAST: i8 = 17;
    // const _NORTH_WEST: i8 = 15;
    // const _SOUTH_EAST: i8 = -15;
    // const _SOUTH_WEST: i8 = -17;
    // const _EAST_NORTH: i8 = 10;
    // const _EAST_SOUTH: i8 = -6;
    // const _WEST_NORTH: i8 = 6;
    // const _WEST_SOUTH: i8 = -10;

    const ALL: [Self; 8] = [
        KnightDirections::NORTH_EAST,
        KnightDirections::NORTH_WEST,
        KnightDirections::SOUTH_EAST,
        KnightDirections::SOUTH_WEST,
        KnightDirections::EAST_NORTH,
        KnightDirections::EAST_SOUTH,
        KnightDirections::WEST_NORTH,
        KnightDirections::WEST_SOUTH,
    ];

    #[inline]
    fn shift(&self, bb: BitBoard) -> BitBoard {
        let NORTH_EAST: i8 = 17;
        let NORTH_WEST: i8 = 15;
        let SOUTH_EAST: i8 = 15;
        let SOUTH_WEST: i8 = 17;
        let EAST_NORTH: i8 = 10;
        let EAST_SOUTH: i8 = 6;
        let WEST_NORTH: i8 = 6;
        let WEST_SOUTH: i8 = 10;

        let NOT_FILE_A = !(FILE_MASKS[Files::A as usize]);
        let NOT_FILE_H = !(FILE_MASKS[Files::H as usize]);
        let NOT_FILE_AB = !(FILE_MASKS[Files::A as usize] | FILE_MASKS[Files::B as usize]);
        let NOT_FILE_GH = !(FILE_MASKS[Files::G as usize] | FILE_MASKS[Files::H as usize]);
        match self {
            KnightDirections::NORTH_EAST => (bb & NOT_FILE_H) << NORTH_EAST,
            KnightDirections::NORTH_WEST => (bb & NOT_FILE_A) << NORTH_WEST,
            KnightDirections::SOUTH_EAST => (bb & NOT_FILE_H) >> SOUTH_EAST,
            KnightDirections::SOUTH_WEST => (bb & NOT_FILE_A) >> SOUTH_WEST,
            KnightDirections::EAST_NORTH => (bb & NOT_FILE_GH) << EAST_NORTH,
            KnightDirections::EAST_SOUTH => (bb & NOT_FILE_GH) >> EAST_SOUTH,
            KnightDirections::WEST_NORTH => (bb & NOT_FILE_AB) << WEST_NORTH,
            KnightDirections::WEST_SOUTH => (bb & NOT_FILE_AB) >> WEST_SOUTH,
        }
    }
}

pub fn get_knight_attacks(knight_square_idx: Square) -> BitBoard {
    let knight_position = SQUARE_MASKS[knight_square_idx];
    let mut knight_attacks = EMPTY_BITBOARD;

    for knight_direction in KnightDirections::ALL {
        knight_attacks |= knight_direction.shift(knight_position);
    }

    knight_attacks
}

// TODO: do just like knight attacks and iterate over the directions
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
