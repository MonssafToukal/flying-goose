use super::{
    defs::Board,
    fen::{FEN_PARSE_FUNCS, FenError, fen_split_string},
    types::{BitBoard, EMPTY_BITBOARD, Sides},
    zobrist::ZobristKey,
};

impl Board {
    pub fn init(&mut self) {
        if self.bb_sides == [EMPTY_BITBOARD; Sides::BOTH] {
            let (white_side, black_side) = self.init_bb_sides();
            self.bb_sides[Sides::WHITE] = white_side;
            self.bb_sides[Sides::BLACK] = black_side;
        }
        self.piece_list = self.get_piece_list();
        self.game_state.zobrist_key = self.init_zobrist_key();
    }

    fn init_bb_sides(&self) -> (BitBoard, BitBoard) {
        let mut white_side = EMPTY_BITBOARD;
        let mut black_side = EMPTY_BITBOARD;
        for (piece_type, (wp, bp)) in self.bb_pieces[Sides::WHITE]
            .iter()
            .zip(self.bb_pieces[Sides::BLACK].iter())
            .enumerate()
        {
            white_side |= *wp;
            black_side |= *bp;
        }
        (white_side, black_side)
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
                    .zobrist_hashmap
                    .piece(Sides::WHITE, piece_type, square_idx);
                white_bitboard &= white_bitboard - 1;
            }
            while black_bitboard != 0 {
                let square_idx = black_bitboard.trailing_zeros() as usize;
                key ^= self
                    .zobrist_hashmap
                    .piece(Sides::BLACK, piece_type, square_idx);
                black_bitboard &= black_bitboard - 1;
            }
        }
        // White to move so we don't include the side_hash
        // Castling should always be true for both sides on both King and Queen side.
        key ^= self.zobrist_hashmap.castling(self.game_state.castling);
        // handle the enpassant file:
        if let Some(enpassant_file_idx) = self.game_state.enpassant {
            key ^= self.zobrist_hashmap.enpassant(enpassant_file_idx);
        }
        if self.game_state.active_color == Sides::BLACK as u8 {
            key ^= self.zobrist_hashmap.side();
        }
        key
    }

    pub fn fen_setup(&mut self, fen: Option<&str>) -> Result<(), FenError> {
        // Step 1. Split the FEN string into 6 parts that we need to parse.
        let fen_parts = fen_split_string(fen)?;
        // expensive operation, should probably do something else here
        let mut new_board = self.clone();
        FEN_PARSE_FUNCS
            .iter()
            .zip(fen_parts.iter())
            .try_for_each(|(fen_parser, part)| fen_parser(&mut new_board, part.as_str()))?;
        *self = new_board;
        Ok(())
    }
}
