#![allow(warnings)]
mod magic;
mod sliders;
use flying_goose::types::print_bb;

use crate::magic::get_rook_magics;


fn main() {
    let (magics, lut) = get_rook_magics();
    for (i, magic_entry) in magics.iter().enumerate() {
        println!("Square {i}");
        println!("{magic_entry}");
        println!("Bits set for magic: {}", magic_entry.number.count_ones());
    }
}
