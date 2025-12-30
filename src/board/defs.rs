use super::{
    piece::Pieces,
    types::{BitBoard, NumOf, Piece, Side, Sides},
};

pub struct Board {
    bb_pieces: [[BitBoard; NumOf::PIECE_TYPES]; Sides::BOTH],
    bb_sides: [BitBoard; Sides::BOTH as usize],
    // move_history: History,
    piece_list: [Piece; NumOf::SQUARES],
}

impl Board {
    pub fn get_pieces(&self, side: Side, piece: Piece) -> BitBoard {
        return self.bb_pieces[side][piece];
    }

    pub fn get_piece_list(&self) -> [Piece; NumOf::SQUARES] {
        let mut piece_list = [Pieces::NONE; NumOf::SQUARES];
        let white_bbs = self.bb_pieces[Sides::WHITE as usize];
        let black_bbs = self.bb_pieces[Sides::BLACK as usize];
        for piece_type in 0..NumOf::PIECE_TYPES {
            let piece_type = Piece::try_from(piece_type).unwrap();
            let white_bitboard = white_bbs[piece_type as usize];
            let black_bitboard = black_bbs[piece_type as usize];
            let mut bit_mask: u64 = 1;

            for square_idx in 0..NumOf::SQUARES {
                let square_idx = square_idx as usize;
                if (bit_mask & white_bitboard) > 0 {
                    piece_list[square_idx] = piece_type;
                }
                bit_mask = bit_mask << 1;
            }
            bit_mask = 1u64;
            for square_idx in 0..NumOf::SQUARES {
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
