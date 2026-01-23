#![allow(warnings)]

pub mod board;

use board::{
    Board, fen::FenError, types::{Files, Pieces, Ranks, Sides, SquareCoord, print_bb}
};

fn main() -> Result<(), FenError> {
    let mut board = Board::init();
    board.fen_setup(None)?;
    print_bb(board.bb_sides[Sides::WHITE] | board.bb_sides[Sides::BLACK]);
    Ok(())
}
