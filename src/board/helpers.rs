use crate::board::Board;
use crate::board::types::Piece;
use crate::board::types::{Side, Square};
use crate::types::SQUARE_MASKS;
use crate::board::types::Pieces;

impl Board {
    pub fn put_piece(&mut self, piece: Piece, side: Side, square_idx: Square) {
        self.bb_pieces[side][piece] |= SQUARE_MASKS[square_idx];
        self.bb_sides[side] |= SQUARE_MASKS[square_idx];
        self.piece_list[square_idx] = piece;
        self.game_state.zobrist_key ^= self.zobrist_hashmap.piece(side, piece, square_idx)
    }

    pub fn remove_piece(&mut self, piece: Piece, side: Side, square_idx: Square) {
        self.bb_pieces[side][piece] &= !SQUARE_MASKS[square_idx];
        self.bb_sides[side] &= !SQUARE_MASKS[square_idx];
        self.piece_list[square_idx] = Pieces::NONE;
        self.game_state.zobrist_key ^= self.zobrist_hashmap.piece(side, piece, square_idx);
    }

    pub fn move_piece(
        &mut self,
        piece: Piece,
        side: Side,
        initial_square: Square,
        final_square: Square,
    ) {
        self.remove_piece(piece, side, initial_square);
        self.put_piece(piece, side, final_square);
    }

    pub fn set_enpassant(&mut self, square: Square) {
        self.game_state.set_enpassant(square);
        let file = square.file();
        self.game_state.zobrist_key ^= self.zobrist_hashmap.enpassant(file);
    }

    pub fn clear_enpassant(&mut self) {
        match self.game_state.enpassant {
            Some(ep_square) => {
                let file = ep_square.file();
                self.game_state.zobrist_key ^= self.zobrist_hashmap.enpassant(file);
                self.game_state.clear_enpassant();
            },
            None => (),
        }
    }

    pub fn update_castling_permissions(&mut self, new_castling_permissions: u8) {
        self.game_state.zobrist_key ^= self.zobrist_hashmap.castling(self.game_state.castling);
        self.game_state.set_castling(new_castling_permissions);
        self.game_state.zobrist_key ^= self.zobrist_hashmap.castling(new_castling_permissions);
    }

    pub fn toggle_side(&mut self) {
        self.game_state.toggle_side();
        self.game_state.zobrist_key ^= self.zobrist_hashmap.side();
    }

    #[inline(always)]
    pub fn get_pawn_direction(side: Side) -> i8 {
        match side {
            Side::Black => -8,
            Side::White => 8,
        }
    }
}
