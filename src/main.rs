#![allow(warnings)]
pub mod board;
pub mod types;
pub mod movement;


use board::{
    Board,
    fen::FenError,
    types::{EMPTY_BITBOARD, Files, Ranks, Sides, SquareCoord},
};

fn main() -> Result<(), FenError> {
    Ok(())
}
