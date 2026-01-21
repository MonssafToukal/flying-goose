use crate::board::{
    Board,
    types::{BitBoard, NumOf, Piece, Pieces, Side, Sides},
};

impl Board {
    pub fn get_pieces(&self, side: Side, piece: Piece) -> BitBoard {
        return self.bb_pieces[side][piece];
    }
    pub fn get_piece_list(&self) -> [Piece; NumOf::SQUARES] {
        let mut piece_list = [Pieces::NONE; NumOf::SQUARES];
        let white_bbs = self.bb_pieces[Sides::WHITE];
        let black_bbs = self.bb_pieces[Sides::BLACK];
        for piece_type in 0..NumOf::PIECE_TYPES {
            let mut white_bitboard = white_bbs[piece_type];
            let mut black_bitboard = black_bbs[piece_type];

            while white_bitboard != 0 {
                let square_idx = white_bitboard.trailing_zeros() as usize;
                piece_list[square_idx] = piece_type;
                white_bitboard &= white_bitboard - 1;
            }

            while black_bitboard != 0 {
                let square_idx = black_bitboard.trailing_zeros() as usize;
                piece_list[square_idx] = piece_type;
                black_bitboard &= black_bitboard - 1;
            }
        }
        piece_list
    }
}
