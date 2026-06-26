#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{EMPTY_BITBOARD, Files, Ranks, Sides, SquareCoord},
};
use movement::{magic::{get_bishop_magics, get_rook_magics}, sliders::{BISHOP_SLIDER, ROOK_SLIDER, Slider, get_all_blockers_subsets}};
use types::{BitBoard, print_bb};

fn main() -> Result<(), FenError> {
    let (rook_magics, rook_lookup_table) = get_rook_magics();
    let (bishop_magics, bishop_lookup_table) = get_bishop_magics();
    println!("{}", rook_magics[0]);

    let mut actual_number_of_elements: u64 = 0;
    rook_lookup_table.iter().for_each(|eligible_moves| {
        if *eligible_moves != EMPTY_BITBOARD {
            actual_number_of_elements += 1;
        }
    });
    print_bb(rook_lookup_table[0]);
    println!("Actual number of elements in lookup table: {}", actual_number_of_elements);

    Ok(())
}
