use board::{defs::Board, fen::{fen_parse_pieces, split_fen_string}, types::{Sides, print_bb, Pieces}};

mod board;
fn main() {
    let mut board = Board::new();
    let fen_parts = split_fen_string(None).unwrap();
    println!("{}", fen_parts[0]);
    fen_parse_pieces(&mut board, fen_parts[0].as_str()).expect("Error occured while parsing the pieces");
    // checking white side:
    for (piece_type, bb_piece) in board.bb_pieces[Sides::WHITE].iter().enumerate() {
        match piece_type {
           Pieces::KING => println!("Piece type: KING"),
           Pieces::QUEEN => println!("Piece type: QUEEN"),
           Pieces::ROOK => println!("Piece type: ROOK"),
           Pieces::BISHOP => println!("Piece type: BISHOP"),
           Pieces::KNIGHT => println!("Piece type: KNIGHT"),
           Pieces::PAWN => println!("Piece type: PAWN"),
            _ => println!("wtf?")
        }
        print_bb(*bb_piece);
        println!("{}", *bb_piece);
    }
}
