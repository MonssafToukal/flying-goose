#![allow(warnings)]
mod magic;

use flying_goose::{board::types::EMPTY_BITBOARD, types::print_bb};

use crate::magic::get_rook_magics;
use magic::{get_bishop_magics, print_magics};

fn main() {
    print_magics("rook");
    print_magics("bishop");
}
