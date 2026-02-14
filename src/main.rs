#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{Files, Ranks, Sides, SquareCoord},
};
use movement::{magic::{generate_slider_blockers_masks, get_all_blockers_subsets}, sliders::{BISHOP_SLIDER, ROOK_SLIDER}};
use types::{BitBoard, SQUARE_MASKS, print_bb};

fn main() -> Result<(), FenError> {
    let rook_blockers = generate_slider_blockers_masks(&ROOK_SLIDER);
    let bishop_blockers = generate_slider_blockers_masks(&BISHOP_SLIDER);

    let bitset: BitBoard = bishop_blockers[35];
    let subsets = get_all_blockers_subsets(bitset);
    println!("{}", subsets.len());
    subsets.iter().for_each(|s| print_bb(*s));
    Ok(())
}
