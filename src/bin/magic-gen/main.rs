#![allow(warnings)]
mod magic;
mod sliders;
use flying_goose::{board::types::EMPTY_BITBOARD, types::print_bb};

use crate::magic::get_rook_magics;


fn main() {
    let (magics, lut) = get_rook_magics();
    let number_attack_moves = lut.into_iter().filter(|bb| *bb != EMPTY_BITBOARD).count();
    let attack_table_used = (number_attack_moves * 8)/(1024);
    println!("table size: {}", attack_table_used)
}
