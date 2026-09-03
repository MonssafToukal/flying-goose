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
// pub type Square = SQ;
pub type CastlingState = u8;

#[repr(u8)]
#[rustfmt::skip]
#[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Debug)]
pub enum Square {
    A1, B1, C1, D1, E1, F1, G1, H1,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A8, B8, C8, D8, E8, F8, G8, H8,
}

impl From<usize> for Square {
    #[rustfmt::skip]
    #[inline(always)]
    fn from(value: usize) -> Self {
        match value {
            0 => Square::A1, 1 => Square::B1, 2 => Square::C1, 3 => Square::D1,
            4 => Square::E1, 5 => Square::F1, 6 => Square::G1, 7 => Square::H1,
            8 => Square::A2, 9 => Square::B2, 10 => Square::C2, 11 => Square::D2,
            12 => Square::E2, 13 => Square::F2, 14 => Square::G2, 15 => Square::H2,
            16 => Square::A3, 17 => Square::B3, 18 => Square::C3, 19 => Square::D3,
            20 => Square::E3, 21 => Square::F3, 22 => Square::G3, 23 => Square::H3,
            24 => Square::A4, 25 => Square::B4, 26 => Square::C4, 27 => Square::D4,
            28 => Square::E4, 29 => Square::F4, 30 => Square::G4, 31 => Square::H4,
            32 => Square::A5, 33 => Square::B5, 34 => Square::C5, 35 => Square::D5,
            36 => Square::E5, 37 => Square::F5, 38 => Square::G5, 39 => Square::H5,
            40 => Square::A6, 41 => Square::B6, 42 => Square::C6, 43 => Square::D6,
            44 => Square::E6, 45 => Square::F6, 46 => Square::G6, 47 => Square::H6,
            48 => Square::A7, 49 => Square::B7, 50 => Square::C7, 51 => Square::D7,
            52 => Square::E7, 53 => Square::F7, 54 => Square::G7, 55 => Square::H7,
            56 => Square::A8, 57 => Square::B8, 58 => Square::C8, 59 => Square::D8,
            60 => Square::E8, 61 => Square::F8, 62 => Square::G8, 63 => Square::H8,
            _ => panic!("unable to convert usize value {} to SQ enum variant", value),
        }
    }
}

impl From<u8> for Square {
    #[rustfmt::skip]
    #[inline(always)]
    fn from(value: u8) -> Self {
        Self::from(value as usize)
    }
}

impl From<u16> for Square {
    #[rustfmt::skip]
    #[inline(always)]
    fn from(value: u16) -> Self {
        Self::from(value as usize)
    }
}

impl Square {
    #[inline(always)]
    pub fn usize(&self) -> usize {
        *self as usize
    }
    #[inline(always)]
    pub fn file(&self) -> usize {
        self.usize() % NumOf::FILES
    }

    #[inline(always)]
    pub fn rank(&self) -> usize {
        self.usize() / NumOf::RANKS
    }

    // This series of function will do no bound checking for speed
    // TODO: implement bound checking if necessary
    #[inline(always)]
    pub fn north(&self) -> Square {
        Square::from(*self as usize + NumOf::FILES)
    }

    #[inline(always)]
    pub fn south(&self) -> Square {
        Square::from(*self as usize - NumOf::FILES)
    }

    #[inline(always)]
    pub fn east(&self) -> Square {
        Square::from(*self as usize + 1)
    }

    #[inline(always)]
    pub fn west(&self) -> Square {
        Square::from(*self as usize - 1)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BySquare<T>([T; NumOf::SQUARES]);
impl<T> BySquare<T>
where
    T: Copy,
{
    pub fn new(value: T) -> Self {
        Self([value; NumOf::SQUARES])
    }

    pub const fn init(values: [T; NumOf::SQUARES]) -> Self {
        Self(values)
    }
}

impl<T> Index<Square> for BySquare<T> {
    type Output = T;

    fn index(&self, index: Square) -> &Self::Output {
        &self.0[index as usize]
    }
}

impl<T> IndexMut<Square> for BySquare<T> {
    fn index_mut(&mut self, index: Square) -> &mut Self::Output {
        &mut self.0[index as usize]
    }
}

// TODO: convert into an enum at some point
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CastlingRight {
    WhiteKingSide = 0x01,
    WhiteQueenSide = 0x02,
    BlackKingSide = 0x04,
    BlackQueenSide = 0x08,
}

impl CastlingRight {
    pub const ALL: u8 = Self::WhiteKingSide as u8
        | Self::WhiteQueenSide as u8
        | Self::BlackKingSide as u8
        | Self::BlackQueenSide as u8;
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
