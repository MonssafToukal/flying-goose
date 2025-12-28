use std::iter::zip;

use super::{
    bitboard::{self, NUM_SQUARES},
    piece::{NUM_PIECE_TYPES, Piece},
};
use bitboard::BitBoard;

pub struct Board {
    bb_pieces: [[BitBoard; NUM_PIECE_TYPES as usize]; Side::Both as usize],
    bb_sides: [BitBoard; Side::Both as usize],
}

impl Board {
    pub fn get_pieces(&self, side: Side, piece: Piece) -> BitBoard {
        return self.bb_pieces[side as usize][piece as usize];
    }
    pub fn get_piece_list(&self) -> [Piece; NUM_SQUARES as usize] {
        let mut piece_list = [Piece::None; NUM_SQUARES as usize];
        let white_bbs = self.bb_pieces[Side::White as usize];
        let black_bbs = self.bb_pieces[Side::Black as usize];
        for piece_type in 0..NUM_PIECE_TYPES {
            let piece_type = Piece::try_from(piece_type).unwrap();
            let white_bitboard = white_bbs[piece_type as usize];
            let black_bitboard = black_bbs[piece_type as usize];
            let mut bit_mask: u64 = 1;

            for square_idx in 0..NUM_SQUARES {
                let square_idx = square_idx as usize;
                if (bit_mask & white_bitboard) > 0 {
                    piece_list[square_idx] = piece_type;
                }
                bit_mask = bit_mask << 1;
            }
            bit_mask = 1u64;
            for square_idx in 0..NUM_SQUARES {
                let square_idx = square_idx as usize;
                if (bit_mask & black_bitboard) > 0 {
                    piece_list[square_idx] = piece_type;
                }
                bit_mask = bit_mask << 1;
            }

        }
        piece_list
    }
}

#[repr(usize)]
pub enum Side {
    White,
    Black,
    Both,
}

// enum GameState {

// }
