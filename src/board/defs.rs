use super::{
    fen::{FEN_PARSE_FUNCS, FenError, fen_split_string},
    history::GameHistory,
    state::GameState,
    types::{BitBoard, EMPTY_BITBOARD, NumOf, Piece, Pieces, Side, Sides},
    zobrist::{Zobrist, ZobristKey},
};

#[derive(Debug, Clone)]
pub struct Board {
    pub bb_pieces: [[BitBoard; NumOf::PIECE_TYPES]; Sides::BOTH],
    pub bb_sides: [BitBoard; Sides::BOTH],
    pub piece_list: [Piece; NumOf::SQUARES],
    pub game_state: GameState,
    pub history: GameHistory,
    pub zobrist_hashmap: Zobrist,
}

impl Board {
    // This is a debug function
    pub fn new() -> Self {
        Board {
            bb_pieces: [[EMPTY_BITBOARD; NumOf::PIECE_TYPES]; Sides::BOTH],
            bb_sides: [EMPTY_BITBOARD; Sides::BOTH],
            piece_list: [Pieces::NONE; NumOf::SQUARES],
            game_state: GameState::new(),
            history: GameHistory::new(),
            zobrist_hashmap: Zobrist::new(None),
        }
    }

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

    pub fn setup(&mut self, fen: Option<&str>) -> Result<(), FenError> {
        // Step 1. Split the FEN string into 6 parts that we need to parse.
        let fen_parts = fen_split_string(fen)?;
        // expensive operation, should probably do something else here
        let mut new_board =  self.clone();
        FEN_PARSE_FUNCS.iter().zip(fen_parts.iter()).try_for_each(|(fen_parser, part)| {
            fen_parser(&mut new_board, part.as_str())
        })?;
        *self = new_board;
        Ok(())
    }
}
