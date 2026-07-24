use crate::{
    board::types::{Files, Side, Sides, Square},
    types::{BitBoard, EMPTY_BITBOARD, FILE_MASKS, NumOf, RANK_MASKS, SQUARE_MASKS, print_bb},
};

#[repr(u8)]
pub enum KingDirections {
    North,
    South,
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
        const NORTH_SHIFT: u8 = 8;
        const SOUTH_SHIFT: u8 = 8;
        const EAST_SHIFT: u8 = 1;
        const WEST_SHIFT: u8 = 1;
        match self {
            KingDirections::North => bb << NORTH_SHIFT,
            KingDirections::South => bb >> SOUTH_SHIFT,
            KingDirections::EAST => bb << EAST_SHIFT,
            KingDirections::WEST => bb >> WEST_SHIFT,
        }
    }
}

// The first cardinal direction mentioned is the one that takes 2 squares
// e.g SouthEast => 2 squares down and one square right
#[derive(Debug)]
pub enum KnightDirections {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    EastNorth,
    EastSouth,
    WestNorth,
    WestSouth,
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
        KnightDirections::NorthEast,
        KnightDirections::NorthWest,
        KnightDirections::SouthEast,
        KnightDirections::SouthWest,
        KnightDirections::EastNorth,
        KnightDirections::EastSouth,
        KnightDirections::WestNorth,
        KnightDirections::WestSouth,
    ];

    #[inline]
    fn shift(&self, bb: BitBoard) -> BitBoard {
        const NORTH_EAST: u8 = 17;
        const NORTH_WEST: u8 = 15;
        const SOUTH_EAST: u8 = 15;
        const SOUTH_WEST: u8 = 17;
        const EAST_NORTH: u8 = 10;
        const EAST_SOUTH: u8 = 6;
        const WEST_NORTH: u8 = 6;
        const WEST_SOUTH: u8 = 10;

        const NOT_FILE_A: BitBoard = !(FILE_MASKS[Files::A as usize]);
        const NOT_FILE_H: BitBoard = !(FILE_MASKS[Files::H as usize]);
        const NOT_FILE_AB: BitBoard =
            !(FILE_MASKS[Files::A as usize] | FILE_MASKS[Files::B as usize]);
        const NOT_FILE_GH: BitBoard =
            !(FILE_MASKS[Files::G as usize] | FILE_MASKS[Files::H as usize]);
        match self {
            KnightDirections::NorthEast => (bb & NOT_FILE_H) << NORTH_EAST,
            KnightDirections::NorthWest => (bb & NOT_FILE_A) << NORTH_WEST,
            KnightDirections::SouthEast => (bb & NOT_FILE_H) >> SOUTH_EAST,
            KnightDirections::SouthWest => (bb & NOT_FILE_A) >> SOUTH_WEST,
            KnightDirections::EastNorth => (bb & NOT_FILE_GH) << EAST_NORTH,
            KnightDirections::EastSouth => (bb & NOT_FILE_GH) >> EAST_SOUTH,
            KnightDirections::WestNorth => (bb & NOT_FILE_AB) << WEST_NORTH,
            KnightDirections::WestSouth => (bb & NOT_FILE_AB) >> WEST_SOUTH,
        }
    }
}

enum PawnDirections {
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
}

impl PawnDirections {
    const ALL: [PawnDirections; 4] = [
        PawnDirections::NorthEast,
        PawnDirections::NorthWest,
        PawnDirections::SouthEast,
        PawnDirections::SouthWest,
    ];

    fn shift(&self, bb: BitBoard, side: Side) -> Option<BitBoard> {
        const NORTH_EAST: u8 = 9;
        const NORTH_WEST: u8 = 7;
        const SOUTH_EAST: u8 = 7;
        const SOUTH_WEST: u8 = 9;
        const NOT_FILE_A: BitBoard = !FILE_MASKS[Files::A as usize];
        const NOT_FILE_H: BitBoard = !FILE_MASKS[Files::H as usize];
        if side == Sides::WHITE {
            match self {
                PawnDirections::NorthEast => Some((bb & NOT_FILE_H) << NORTH_EAST),
                PawnDirections::NorthWest => Some((bb & NOT_FILE_A) << NORTH_WEST),
                _ => None,
            }
        } else if side == Sides::BLACK {
            match self {
                PawnDirections::SouthEast => Some((bb & NOT_FILE_H) >> SOUTH_EAST),
                PawnDirections::SouthWest => Some((bb & NOT_FILE_A) >> SOUTH_WEST),
                _ => None,
            }
        } else {
            None
        }
    }
}

pub fn get_pawn_attacks(square_idx: Square, color: Side) -> BitBoard {
    let pawn_position = SQUARE_MASKS[square_idx];
    let mut pawn_attack = EMPTY_BITBOARD;
    PawnDirections::ALL.iter().for_each(|d| {
        if let Some(attack) = d.shift(pawn_position, color) {
            pawn_attack |= attack;
        }
    });
    pawn_attack
}

pub fn get_knight_attacks(knight_square_idx: Square) -> BitBoard {
    let knight_position = SQUARE_MASKS[knight_square_idx];
    let mut knight_attacks = EMPTY_BITBOARD;

    for knight_direction in KnightDirections::ALL {
        knight_attacks |= knight_direction.shift(knight_position);
    }

    knight_attacks
}

pub fn get_king_attacks(king_square_idx: Square) -> BitBoard {
    const NOT_RANK_8: BitBoard = !RANK_MASKS[NumOf::RANKS - 1];
    const NOT_RANK_1: BitBoard = !RANK_MASKS[0];
    let king_position = SQUARE_MASKS[king_square_idx];
    let mut king_attacks = get_king_attacks_east_west(king_position);

    // North direction:
    king_attacks |= KingDirections::North.shift(king_position & NOT_RANK_8);

    // South direction:
    king_attacks |= KingDirections::South.shift(king_position & NOT_RANK_1);

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
