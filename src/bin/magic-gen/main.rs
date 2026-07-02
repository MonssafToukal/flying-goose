#![allow(warnings)]
mod magic;
mod sliders;
use flying_goose::types::print_bb;

use crate::magic::get_rook_magics;


fn main() {
    let (magics, lut) = get_rook_magics();
    magics.iter().zip(lut).for_each(|(m, bb)| {
        println!("Magic: {}\n bitboard: ", m);
        print_bb(bb)});
}
