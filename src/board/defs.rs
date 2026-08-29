use crate::types::BitBoard;
use crate::{
    board::{
        Board,
        types::{Piece, Pieces, Side},
    },
    types::NumOf,
};

use super::types::{BySquare, Square};

impl Board {
    #[inline]
    pub fn get_pieces(&self, side: Side, piece: Piece) -> BitBoard {
        return self.bb_pieces[side][piece];
    }

    pub fn build_piece_list(&self) -> BySquare<Piece> {
        let mut piece_list = BySquare::new(Pieces::NONE);
        let white_bbs = self.bb_pieces[Side::White];
        let black_bbs = self.bb_pieces[Side::Black];
        for piece_type in 0..NumOf::PIECE_TYPES {
            let mut white_bitboard = white_bbs[piece_type];
            let mut black_bitboard = black_bbs[piece_type];

            while white_bitboard != 0 {
                let square_idx = Square::from(white_bitboard.trailing_zeros() as usize);
                piece_list[square_idx] = piece_type;
                white_bitboard &= white_bitboard - 1;
            }

            while black_bitboard != 0 {
                let square_idx = Square::from(black_bitboard.trailing_zeros() as usize);
                piece_list[square_idx] = piece_type;
                black_bitboard &= black_bitboard - 1;
            }
        }
        piece_list
    }

    #[inline(always)]
    pub fn get_current_player(&self) -> Side {
        self.game_state.active_color
    }

    #[inline(always)]
    pub fn get_opponent(&self) -> Side {
        self.game_state.active_color.other()
    }
}
