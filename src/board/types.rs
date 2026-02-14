use std::fmt::Display;

use crate::types::BitBoard;
use crate::types::NumOf;
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

#[derive(Debug)]
pub enum BoardError {
    FileOutOfBound,
    RankOutOfBound,
    SquareOutOfBound,
}

impl Display for BoardError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            BoardError::FileOutOfBound => "File does not exist or is out of bound",
            BoardError::RankOutOfBound => "Rank does not exist or is out of bound",
            BoardError::SquareOutOfBound => "Square does not exist or is out of bound",
        };
        write!(f, "{err}")
    }
}

impl Files {
    pub fn next(&self, x_direction: i8) -> Result<Self, BoardError> {
        let next_file_number = *self as i16 + x_direction as i16;
        if next_file_number < 0 || next_file_number >= NumOf::FILES as i16 {
            return Err(BoardError::FileOutOfBound);
        }
        let next_file_number = next_file_number as u8;
        Ok(Self::try_from(next_file_number).unwrap())
    }
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

impl Ranks {
    pub fn next(&self, x_direction: i8) -> Result<Self, BoardError> {
        let next_rank_number = *self as i16 + x_direction as i16;
        if next_rank_number < 0 || next_rank_number >= NumOf::RANKS as i16 {
            return Err(BoardError::RankOutOfBound);
        }
        let next_rank_number = next_rank_number as u8;
        Ok(Self::try_from(next_rank_number).unwrap())
    }
}

// Direction is a vector (x,y) to denote the direction a sliding piece can take
pub type Direction = (i8, i8);
pub const MAX_DIRECTIONS: usize = 4;

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

    pub fn next(self, direction: Direction) -> Result<Self, BoardError> {
        let (x_direction, y_direction) = direction;
        let next_file = self.file.next(x_direction)?;
        let next_rank = self.rank.next(y_direction)?;
        Ok(SquareCoord {
            file: next_file,
            rank: next_rank,
        })
    }
}

impl TryFrom<u8> for SquareCoord {
    type Error = BoardError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value >= NumOf::SQUARES as u8 {
            return Err(BoardError::SquareOutOfBound);
        }
        let rank_number = value / NumOf::RANKS as u8;
        let file_number = value - (rank_number * NumOf::RANKS as u8);
        let rank = match Ranks::try_from(rank_number) {
            Ok(r) => r,
            Err(_) => return Err(BoardError::RankOutOfBound),
        };
        let file = match Files::try_from(file_number) {
            Ok(r) => r,
            Err(_) => return Err(BoardError::FileOutOfBound),
        };
        Ok(SquareCoord {
            file: file,
            rank: rank,
        })
    }
}
