#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{Files, Ranks, Sides, SquareCoord},
};
use movement::magic::{generate_rook_blockers_masks, get_all_blockers_subsets};
use types::{BitBoard, SQUARE_MASKS, print_bb};

fn main() -> Result<(), FenError> {
    let rook_blockers = generate_rook_blockers_masks();

    let bitset: BitBoard = rook_blockers[0];
    let subsets = get_all_blockers_subsets(bitset);
    println!("{}", subsets.len());
    subsets.iter().for_each(|s| print_bb(*s));
    Ok(())
}
