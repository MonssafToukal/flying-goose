#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{Files, Ranks, Sides, SquareCoord},
};
use movement::sliders::{BISHOP_SLIDER, ROOK_SLIDER, generate_slider_blockers_masks, get_all_blockers_subsets};
use types::{BitBoard, print_bb};

fn main() -> Result<(), FenError> {
    let rook_blockers = generate_slider_blockers_masks(&ROOK_SLIDER);
    let bishop_blockers = generate_slider_blockers_masks(&BISHOP_SLIDER);
    let square = SquareCoord::try_from(0u8).unwrap();
    println!("Rook blockers for square A1");
    print_bb(rook_blockers[0]);

    let bitset: BitBoard = rook_blockers[0];
    let subsets = get_all_blockers_subsets(bitset);
    let subset = subsets[1902];

    println!("first blocker configuration for rook on A1");
    print_bb(subset);

    let rook_moves = ROOK_SLIDER.get_moves(square, subset);
    println!("eligible rook moves");
    print_bb(rook_moves);
    Ok(())
}
