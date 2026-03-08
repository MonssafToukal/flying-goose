#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{Files, Ranks, Sides, SquareCoord},
};
use movement::sliders::{BISHOP_SLIDER, ROOK_SLIDER, Slider, get_all_blockers_subsets};
use types::{BitBoard, print_bb};

fn main() -> Result<(), FenError> {
    let rook_blockers = ROOK_SLIDER.get_all_blockers();
    let bishop_blockers = BISHOP_SLIDER.get_all_blockers();
    let blockers = bishop_blockers;
    let square = SquareCoord::try_from(0u8).unwrap();

    println!("blockers for square A1");
    print_bb(blockers[0]);

    let bitset: BitBoard = bishop_blockers[0];
    let subsets = get_all_blockers_subsets(bitset);
    let subset = subsets[34];

    println!("blocker configuration subset:");
    print_bb(subset);

    let moves = BISHOP_SLIDER.get_moves(square, subset);
    println!("eligible moves");
    print_bb(moves);
    Ok(())
}
