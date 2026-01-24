use std::fmt::Display;

use crate::types::{BitBoard, NumOf};
use num_enum::{FromPrimitive, TryFromPrimitive};

pub const EMPTY_BITBOARD: BitBoard = 0;

pub const MAX_GAME_MOVES: u64 = 2048;

// 50 full moves equates to 100 half moves
pub const FIFTY_MOVE_RULE: u8 = 100;

pub type Piece = usize;
pub type Square = usize;
pub type Side = usize;
pub type CastlingState = u8;

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

#[repr(u8)]
pub enum CastlingRight {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8,
}

#[derive(Debug, PartialEq, Clone, Copy, TryFromPrimitive)]
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

#[derive(Debug, PartialEq, Clone, Copy, TryFromPrimitive)]
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

#[derive(Debug, Copy, Clone)]
pub struct SquareCoord {
    pub file: Files,
    pub rank: Ranks,
}

impl SquareCoord {
    pub fn to_usize(&self) -> usize {
        let file = usize::from(self.file as u8);
        let rank = usize::from(self.rank as u8);

        (rank * NumOf::RANKS) + file
    }
}
