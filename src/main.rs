#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{Files, Ranks, Sides, SquareCoord},
};
use movement::sliders::{ROOK_BLOCKERS_MASK, get_all_blockers_subsets};
use types::{BitBoard, SQUARE_MASKS, print_bb};

fn main() -> Result<(), FenError> {
    let bitset: BitBoard = ROOK_BLOCKERS_MASK[0];
    let subsets = get_all_blockers_subsets(bitset);
    println!("{}", subsets.len());
    subsets.iter().for_each(|s| print_bb(*s));
    Ok(())
}
