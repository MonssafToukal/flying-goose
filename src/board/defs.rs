use super::{
    fen::{FEN_PARSE_FUNCS, FenError, fen_split_string},
    history::GameHistory,
    state::GameState,
    types::{
        BitBoard, EMPTY_BITBOARD, NumOf, Piece, Pieces, SQUARE_MASKS, Side, Sides, SquareCoord,
    },
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
