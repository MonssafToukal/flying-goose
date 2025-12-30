use super::{
    piece::Pieces, state::{GameState, NO_ENPASSANT}, types::{BitBoard, NumOf, Piece, Side, Sides}, zobrist::{Zobrist, ZobristKey}
};

pub struct Board {
    bb_pieces: [[BitBoard; NumOf::PIECE_TYPES]; Sides::BOTH],
    bb_sides: [BitBoard; Sides::BOTH],
    // move_history: History,
    piece_list: [Piece; NumOf::SQUARES],
    game_state: GameState,
    zobrist_keys: Zobrist,
}

impl Board {
    pub fn get_pieces(&self, side: Side, piece: Piece) -> BitBoard {
        return self.bb_pieces[side][piece];
    }

    pub fn init_zobrist_key(&self) -> ZobristKey {
        let mut key = 0u64;
        let white_bbs = self.bb_pieces[Sides::WHITE];
        let black_bbs = self.bb_pieces[Sides::BLACK];
        for (piece_type, (w, b)) in white_bbs.iter().zip(black_bbs.iter()).enumerate() {
            let mut white_bitboard = *w;
            let mut black_bitboard = *b;
            while white_bitboard != 0 {
                let square_idx = white_bitboard.trailing_zeros() as usize;
                key ^= self
                    .zobrist_keys
                    .piece(Sides::WHITE, piece_type, square_idx);
                white_bitboard &= white_bitboard - 1;
            }
            while black_bitboard != 0 {
                let square_idx = black_bitboard.trailing_zeros() as usize;
                key ^= self
                    .zobrist_keys
                    .piece(Sides::BLACK, piece_type, square_idx);
                black_bitboard &= black_bitboard - 1;
            }
        }
        // White to move so we don't include the side_hash
        // Castling should always be true for both sides on both King and Queen side.
        key ^= self.zobrist_keys.castling(self.game_state.castling);

        // handle the enpassant file:
        if self.game_state.enpassant != NO_ENPASSANT {
            key ^= self.zobrist_keys.enpassant(self.game_state.enpassant);
        }
        if self.game_state.side_to_move == Sides::BLACK {
            key ^= self.zobrist_keys.side();
        }
        key
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
