use crate::board::Board;
use crate::board::fen::{FEN_PARSE_FUNCS, FenError, fen_split_string};
use crate::board::history::GameHistory;
use crate::board::state::GameState;
use crate::board::{
    types::{Pieces, Side},
    zobrist::{Zobrist, ZobristKey},
};
use crate::types::{BitBoard, EMPTY_BITBOARD, NumOf};

use super::types::{BySide, BySquare, Square};

impl Board {
    fn new() -> Self {
        Board {
            bb_pieces: BySide::new([EMPTY_BITBOARD; NumOf::PIECE_TYPES]),
            bb_sides: BySide::new(EMPTY_BITBOARD),
            piece_list: BySquare::new(Pieces::NONE),
            game_state: GameState::new(),
            history: GameHistory::new(),
            zobrist_hashmap: Zobrist::new(None),
        }
    }
    pub fn init(&mut self) {
        let (white_side, black_side) = self.init_bb_sides();
        self.bb_sides[Side::White] = white_side;
        self.bb_sides[Side::Black] = black_side;
        self.piece_list = self.build_piece_list();
        self.game_state.zobrist_key = self.init_zobrist_key();
    }

    fn init_bb_sides(&self) -> (BitBoard, BitBoard) {
        let mut white_side = EMPTY_BITBOARD;
        let mut black_side = EMPTY_BITBOARD;
        for (wp, bp) in self.bb_pieces[Side::White]
            .iter()
            .zip(self.bb_pieces[Side::Black].iter())
        {
            white_side |= *wp;
            black_side |= *bp;
        }
        (white_side, black_side)
    }

    fn init_zobrist_key(&self) -> ZobristKey {
        let mut key = 0u64;
        let white_bbs = self.bb_pieces[Side::White];
        let black_bbs = self.bb_pieces[Side::Black];
        for (piece_type, (w, b)) in white_bbs.iter().zip(black_bbs.iter()).enumerate() {
            let mut white_bitboard = *w;
            let mut black_bitboard = *b;
            while white_bitboard != 0 {
                let square_idx = Square::from(white_bitboard.trailing_zeros() as usize);
                key ^= self
                    .zobrist_hashmap
                    .piece(Side::White, piece_type, square_idx);
                white_bitboard &= white_bitboard - 1;
            }
            while black_bitboard != 0 {
                let square_idx = Square::from(black_bitboard.trailing_zeros() as usize);
                key ^= self
                    .zobrist_hashmap
                    .piece(Side::Black, piece_type, square_idx);
                black_bitboard &= black_bitboard - 1;
            }
        }
        // White to move so we don't include the side_hash
        // Castling should always be true for both sides on both King and Queen side.
        key ^= self.zobrist_hashmap.castling(self.game_state.castling);
        // handle the enpassant file:
        if let Some(enpassant_square) = self.game_state.enpassant {
            let enpassant_file = (enpassant_square.file()) as usize;
            key ^= self.zobrist_hashmap.enpassant(enpassant_file);
        }
        if self.game_state.active_color == Side::Black {
            key ^= self.zobrist_hashmap.side();
        }
        key
    }

    // TODO: write this method without relying on cloning the board
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

        self.init();
        Ok(())
    }
}
