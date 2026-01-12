use std::fmt::Display;

pub type BitBoard = u64;

pub fn print_bb(bitboard: BitBoard) -> () {
    const LAST_SQUARE_BIT: u64 = 63;
    // rank 0 is the h-rank
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask: u64 = 1u64 << (LAST_SQUARE_BIT - (rank * 8) - file);
            if mask & bitboard != 0 {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!();
    }
}

pub const EMPTY_BITBOARD: BitBoard  = 0;

pub const MAX_GAME_MOVES: u64 = 2048;

// 50 full moves equates to 100 half moves
pub const FIFTY_MOVE_RULE: u8 = 100;

pub type Piece = usize;
pub type Square = usize;
pub type Side = usize;
pub type CastlingState = u8;
pub type EnpassantSquareIdx = u8;

#[derive(Debug, PartialEq)]
pub struct Pieces;
impl Pieces {
    pub const KING: Piece = 0;
    pub const QUEEN: Piece = 1;
    pub const ROOK: Piece = 2;
    pub const BISHOP: Piece = 3;
    pub const KNIGHT: Piece = 4;
    pub const PAWN: Piece = 5;
    pub const NONE: Piece = 6;
}

pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub struct NumOf;
impl NumOf {
    pub const SQUARES: usize = 64;
    pub const PIECES_PER_SIDE: usize = 16;
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_STATES: usize = 16;
    pub const ENPASSANT_FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const FILES: usize = 8;
}

#[repr(u8)]
pub enum CastlingRight {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Files {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

#[derive(Debug, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Ranks {
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
}

pub const SQUARE_MASKS: [u64; NumOf::SQUARES] = generate_square_masks();

const fn generate_square_masks() -> [u64; NumOf::SQUARES] {
    let mut square_masks = [0u64; NumOf::SQUARES];
    let mut i = 0;
    while i < NumOf::SQUARES {
        square_masks[i] = 1u64 << i;
        i += 1;
    }
    square_masks
}
