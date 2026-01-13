#![allow(warnings)]
use board::{
    defs::Board,
    fen::{fen_parse_pieces, fen_split_string},
    types::{NumOf, Pieces, Sides, print_bb},
};
mod board;

fn main() {
    let mut board = Board::new();
    let fen_parts = fen_split_string(None).unwrap();
    println!("{}", fen_parts[0]);
    fen_parse_pieces(&mut board, fen_parts[0].as_str())
        .expect("Error occured while parsing the pieces");
    // checking white side:
    for (piece_type, bb_piece) in board.bb_pieces[Sides::WHITE].iter().enumerate() {
        match piece_type {
            Pieces::KING => println!("Piece type: KING"),
            Pieces::QUEEN => println!("Piece type: QUEEN"),
            Pieces::ROOK => println!("Piece type: ROOK"),
            Pieces::BISHOP => println!("Piece type: BISHOP"),
            Pieces::KNIGHT => println!("Piece type: KNIGHT"),
            Pieces::PAWN => println!("Piece type: PAWN"),
            _ => println!("wtf?"),
        }
        print_bb(*bb_piece);
    }
}
