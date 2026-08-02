use crate::{
    board::{
        Board,
        types::{Piece, Pieces, Side, Square},
    },
    types::SQUARE_MASKS,
};

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

    pub fn set_enpassant_move(&mut self, square: Square) {
        let file = (square % 8) as usize;
        self.game_state.zobrist_key ^= self.zobrist_hashmap.enpassant(file);
    }
}

#[derive(Clone, Copy)]
pub struct Move(u16);

impl Move {
    const TO_SQUARE_BIT_SHIFT: u8 = 6;
    const FLAGS_BIT_SHIFT: u8 = 12;
    const FROM_SQUARE_MASK: u16 = 0x003F;
    const TO_SQUARE_MASK: u16 = 0x003F << Self::TO_SQUARE_BIT_SHIFT;
    const FLAGS_MASK: u16 = 0x0F << Self::FLAGS_BIT_SHIFT;

    pub fn from_square(&self) -> Square {
        (self.0 & Self::FROM_SQUARE_MASK) as Square
    }

    pub fn to_square(&self) -> Square {
        (self.0  & Self::TO_SQUARE_MASK) as Square
    }

    pub fn flags(&self) -> MoveFlag {
        match (self.0 & Self::FLAGS_MASK) as u8 {
            0b0000 => MoveFlag::Quiet,
            0b0001 => MoveFlag::DoublePawnPush,
            0b0010 => MoveFlag::KingSideCastle,
            0b0011 => MoveFlag::QueenSideCastle,
            0b0100 => MoveFlag::Capture,
            0b0101 => MoveFlag::EpCapture,
            0b0110 => unreachable!(),
            0b0111 => unreachable!(),
            0b1000 => MoveFlag::KnightPromotion,
            0b1001 => MoveFlag::BishopPromotion,
            0b1010 => MoveFlag::RookPromotion,
            0b1011 => MoveFlag::QueenPromotion,
            0b1100 => MoveFlag::KnightCapturePromotion,
            0b1101 => MoveFlag::BishopCapturePromotion,
            0b1110 => MoveFlag::RookCapturePromotion,
            0b1111 => MoveFlag::QueenCapturePromotion,
            _ => unreachable!(),
        }
    }
}


#[repr(u8)]
#[derive(Clone, Copy, Debug)]
pub enum MoveFlag {
    Quiet = 0b0000,
    DoublePawnPush = 0b0001,
    KingSideCastle = 0b0010,
    QueenSideCastle = 0b0011,
    Capture = 0b0100,
    EpCapture = 0b0101,
    KnightPromotion = 0b1000,
    BishopPromotion = 0b1001,
    RookPromotion = 0b1010,
    QueenPromotion = 0b1011,
    KnightCapturePromotion = 0b1100,
    BishopCapturePromotion = 0b1101,
    RookCapturePromotion = 0b1110,
    QueenCapturePromotion = 0b1111,
}
