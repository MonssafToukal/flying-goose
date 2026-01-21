#![allow(warnings)]

use board::{
    Board,
    types::{Sides, print_bb},
};
pub mod board;

fn main() {
    let mut board = Board::init();
    board.fen_setup(None);
    print_bb(board.bb_sides[Sides::WHITE]);
}
