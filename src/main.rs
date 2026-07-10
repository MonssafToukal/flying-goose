#![allow(warnings)]
pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{EMPTY_BITBOARD, Files, Ranks, Sides, SquareCoord},
};

fn main() -> Result<(), FenError> {
    Ok(())
}
