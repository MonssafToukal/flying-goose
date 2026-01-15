#![allow(warnings)]
use board::{
    defs::Board,
    fen::{fen_parse_pieces, fen_split_string},
    types::{BitBoard, NumOf, Pieces, Sides, print_bb},
};
mod board;


fn main() {
    let white_pawn_masks: BitBoard = ((1u64 << 16) - 1) & !((1u64 << 8)-1);
    print_bb(white_pawn_masks);
    println!("");
    let black_pawn_masks: BitBoard = (1u64 << 56) - 1 & ! ((1u64 << 48) - 1);
    print_bb(black_pawn_masks);
}
