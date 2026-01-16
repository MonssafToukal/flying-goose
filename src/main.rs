#![allow(warnings)]
use board::{
    defs::Board,
    fen::{fen_parse_castling_rights, fen_parse_colour, fen_parse_enpassant, fen_parse_pieces, fen_split_string},
    types::{BitBoard, CastlingRight, EMPTY_BITBOARD, NumOf, Pieces, Sides, print_bb},
};
mod board;


fn main() {
    let fen_string_test_cases = [
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 1 3",
        "k7/8/8/8/8/8/8/7K w - - 49 100",
        "4k3/8/8/8/8/8/8/4K3 w - - 100 150",
        "k7/8/8/8/8/8/8/7K w - - 0005 10",
        "8/8/8/8/8/8/8/8 w - - 0",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 12.5 1",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq -",
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - -1 1"
    ];
    for fen_string in fen_string_test_cases {
        let mut board =  Board::new();
        let setup_result = board.setup(Some(fen_string));
        match setup_result {
            Ok(_) => println!("fen string: {fen_string} setup passed"),
            Err(e) => println!("fen string {fen_string}\n Error occured: {e}"),
        }
    }
}
