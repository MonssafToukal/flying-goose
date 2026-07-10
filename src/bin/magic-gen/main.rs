#![allow(warnings)]
mod magic;

use flying_goose::{
    board::types::EMPTY_BITBOARD,
    movement::sliders::{BISHOP_SLIDER, ROOK_SLIDER},
    types::{BitBoard, print_bb},
};

use crate::magic::get_slider_magics;
use magic::print_magics;

fn main() {
    let (magic_entries, rook_global_table) = get_slider_magics(&ROOK_SLIDER);
    let size_kb = (rook_global_table.len() * std::mem::size_of::<BitBoard>()) as f64 / 1024.0;
    println!("Rook global table size: {:.2} Kb", size_kb);
}
