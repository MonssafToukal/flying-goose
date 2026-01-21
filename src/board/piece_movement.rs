use crate::board::{
    Board,
    types::{Piece, Pieces, SQUARE_MASKS, Side, SquareCoord},
};

impl Board {
    pub fn put_piece(&mut self, piece: Piece, side: Side, square: SquareCoord) {
        let square_idx = square.to_usize();
        self.bb_pieces[side][piece] |= SQUARE_MASKS[square_idx];
        self.bb_sides[side] |= SQUARE_MASKS[square_idx];
        self.piece_list[square_idx] = piece;
        self.game_state.zobrist_key ^= self.zobrist_hashmap.piece(side, piece, square_idx)
    }

    pub fn remove_piece(&mut self, piece: Piece, side: Side, square: SquareCoord) {
        let square_idx = square.to_usize();
        self.bb_pieces[side][piece] &= !SQUARE_MASKS[square_idx];
        self.bb_sides[side] &= !SQUARE_MASKS[square_idx];
        self.piece_list[square_idx] = Pieces::NONE;
        self.game_state.zobrist_key ^= self.zobrist_hashmap.piece(side, piece, square_idx);
    }

    pub fn move_piece(
        &mut self,
        piece: Piece,
        side: Side,
        initial_square: SquareCoord,
        final_square: SquareCoord,
    ) {
        self.remove_piece(piece, side, initial_square);
        self.put_piece(piece, side, final_square);
    }

    pub fn set_enpassant_move(&mut self, square: SquareCoord) {
        self.game_state.zobrist_key ^= self.zobrist_hashmap.enpassant(square.file);
    }
}
