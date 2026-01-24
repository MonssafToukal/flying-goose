#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{Board, fen::FenError, types::Sides};
use types::print_bb;

fn main() -> Result<(), FenError> {
    let mut board = Board::init();
    board.fen_setup(None)?;
    print_bb(board.bb_sides[Sides::WHITE] | board.bb_sides[Sides::BLACK]);
    Ok(())
}
