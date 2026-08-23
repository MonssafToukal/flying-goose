use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

use crate::types::NumOf;
use num_enum::TryFromPrimitive;

pub const MAX_GAME_MOVES: u64 = 2048;

// 50 full moves equates to 100 half moves
pub const FIFTY_MOVE_RULE: u8 = 100;

pub type Piece = usize;
pub type Square = usize;
pub type CastlingState = u8;

#[repr(usize)]
#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum SQ {
    A1, A2, A3, A4, A5, A6, A7, A8,
    B1, B2, B3, B4, B5, B6, B7, B8,
    C1, C2, C3, C4, C5, C6, C7, C8,
    D1, D2, D3, D4, D5, D6, D7, D8,
    E1, E2, E3, E4, E5, E6, E7, E8,
    F1, F2, F3, F4, F5, F6, F7, F8,
    G1, G2, G3, G4, G5, G6, G7, G8,
    H1, H2, H3, H4, H5, H6, H7, H8,
}

impl From<usize> for SQ {
    #[rustfmt::skip]
    #[inline(always)]
    fn from(value: usize) -> Self {
        match value {
            0 => SQ::A1, 1 => SQ::B1, 2 => SQ::C1, 3 => SQ::D1,
            4 => SQ::E1, 5 => SQ::F1, 6 => SQ::G1, 7 => SQ::H1,
            8 => SQ::A2, 9 => SQ::B2, 10 => SQ::C2, 11 => SQ::D2,
            12 => SQ::E2, 13 => SQ::F2, 14 => SQ::G2, 15 => SQ::H2,
            16 => SQ::A3, 17 => SQ::B3, 18 => SQ::C3, 19 => SQ::D3,
            20 => SQ::E3, 21 => SQ::F3, 22 => SQ::G3, 23 => SQ::H3,
            24 => SQ::A4, 25 => SQ::B4, 26 => SQ::C4, 27 => SQ::D4,
            28 => SQ::E4, 29 => SQ::F4, 30 => SQ::G4, 31 => SQ::H4,
            32 => SQ::A5, 33 => SQ::B5, 34 => SQ::C5, 35 => SQ::D5,
            36 => SQ::E5, 37 => SQ::F5, 38 => SQ::G5, 39 => SQ::H5,
            40 => SQ::A6, 41 => SQ::B6, 42 => SQ::C6, 43 => SQ::D6,
            44 => SQ::E6, 45 => SQ::F6, 46 => SQ::G6, 47 => SQ::H6,
            48 => SQ::A7, 49 => SQ::B7, 50 => SQ::C7, 51 => SQ::D7,
            52 => SQ::E7, 53 => SQ::F7, 54 => SQ::G7, 55 => SQ::H7,
            56 => SQ::A8, 57 => SQ::B8, 58 => SQ::C8, 59 => SQ::D8,
            60 => SQ::E8, 61 => SQ::F8, 62 => SQ::G8, 63 => SQ::H8,
            _ => panic!("unable to convert usize value {} to SQ enum variant", value),
        }
    }
}

pub struct BySquare<T>([T; NumOf::SQUARES]);
impl<T> BySquare<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self([value; NumOf::SQUARES])
    }
}

impl<T> Index<SQ> for BySquare<T> {
    type Output = T;

    fn index(&self, index: SQ) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<SQ> for BySquare<T> {
    fn index_mut(&mut self, index: SQ) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

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

#[derive(Debug, Clone, Copy)]
pub struct BySide<T>([T; NumOf::SIDES]);
impl<T> BySide<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self([value; NumOf::SIDES])
    }
}

impl<T> Index<Side> for BySide<T> {
    type Output = T;

    #[inline(always)]
    fn index(&self, index: Side) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Side> for BySide<T> {
    #[inline(always)]
    fn index_mut(&mut self, index: Side) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Side {
    Black = 0x00,
    White = 0x01,
}

impl From<usize> for Side {
    fn from(value: usize) -> Self {
        match value {
            0 => Side::Black,
            1 => Side::White,
            _ => panic!(
                "unable to  convert usize value {} to Side enum variant",
                value
            ),
        }
    }
}

impl Side {
    #[inline(always)]
    pub fn other(&self) -> Self {
        match self {
            Side::Black => Side::White,
            Side::White => Side::Black,
        }
    }

    #[inline(always)]
    pub fn toggle_mut(&mut self) {
        *self = self.other();
    }

    #[inline(always)]
    pub fn u8(&self) -> u8 {
        *self as u8
    }
}

#[repr(u8)]
pub enum CastlingRight {
    WhiteKingSide = 0x01,
    WhiteQueenSide = 0x02,
    BlackKingSide = 0x04,
    BlackQueenSide = 0x08,
}

// TODO: replace tryfrom primitive with from instead like Side enum
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
