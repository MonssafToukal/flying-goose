#![allow(warnings)]
use board::{
    defs::Board,
    fen::{fen_parse_castling_rights, fen_parse_colour, fen_parse_enpassant, fen_parse_pieces, fen_split_string},
    types::{BitBoard, CastlingRight, EMPTY_BITBOARD, NumOf, Pieces, Sides, print_bb},
};
mod board;


fn main() {
    // Let's test the parsing function for the fen strings
    // By default, it will use the default position
    let fen_parts = fen_split_string(None).unwrap();
    let board =  &mut Board::new();
    let fen_parts = fen_split_string(None).unwrap();
    fen_parse_pieces(board, fen_parts[0].as_str()).unwrap();
    fen_parse_colour(board, fen_parts[1].as_str()).expect("could not parse the colour for the fen string");
    assert_eq!(board.game_state.active_color, Sides::WHITE as u8);
    // test castling rights
    fen_parse_castling_rights(board, fen_parts[2].as_str()).unwrap();
    let all_castling_rights = CastlingRight::BlackKingSide as u8
        | CastlingRight::BlackQueenSide as u8
        | CastlingRight::WhiteKingSide as u8
        | CastlingRight::WhiteQueenSide as u8;
    assert_eq!(all_castling_rights, board.game_state.castling);
    // test enpassant
    fen_parse_enpassant(board, fen_parts[3].as_str()).unwrap();
}
