use std::fmt::Display;

use crate::{
    board::{
        Board,
        state::GameState,
        types::{Piece, Pieces, Side, Square},
    },
    types::SQUARE_MASKS,
};

impl Board {
    pub fn make(&mut self, chess_move: Move) -> () {
        let mut new_game_state = self.game_state.clone();
        new_game_state.toggle_side();
        let from_square: Square = chess_move.from_square();
        let dest_square: Square = chess_move.dest_square();
        // TODO: handle this better than with unwrap
        let move_flags = chess_move.flags().unwrap();


        todo!()
    }

    pub fn unmake(&mut self, piece: Piece, from_square: Square, to_square: Square) -> () {
        todo!()
    }
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
    const DEST_SQUARE_BIT_SHIFT: u8 = 6;
    const FLAGS_BIT_SHIFT: u8 = 12;
    const FROM_SQUARE_MASK: u16 = 0x003F;
    const DEST_SQUARE_MASK: u16 = 0x003F << Self::DEST_SQUARE_BIT_SHIFT;
    const FLAGS_MASK: u16 = 0x0F << Self::FLAGS_BIT_SHIFT;

    pub fn new(from_square: Square, dest_square: Square, flags: u8) -> Self {
        let chess_move: u16 = from_square as u16
            | (dest_square << Self::DEST_SQUARE_BIT_SHIFT) as u16
            | (flags << Self::FLAGS_BIT_SHIFT) as u16;
        Move(chess_move)
    }
    pub fn from_square(&self) -> Square {
        (self.0 & Self::FROM_SQUARE_MASK) as Square
    }

    pub fn dest_square(&self) -> Square {
        (self.0 & Self::DEST_SQUARE_MASK) as Square
    }

    pub fn flags(&self) -> Result<MoveFlag, InvalidMoveFlag> {
        let move_flag_values = (self.0 & Self::FLAGS_MASK) as u8;
        MoveFlag::try_from(move_flag_values)
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
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

impl TryFrom<u8> for MoveFlag {
    type Error = InvalidMoveFlag;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0b0000 => Ok(MoveFlag::Quiet),
            0b0001 => Ok(MoveFlag::DoublePawnPush),
            0b0010 => Ok(MoveFlag::KingSideCastle),
            0b0011 => Ok(MoveFlag::QueenSideCastle),
            0b0100 => Ok(MoveFlag::Capture),
            0b0101 => Ok(MoveFlag::EpCapture),
            0b0110 => Err(Self::Error {
                invalid_flag_state: value,
            }),
            0b0111 => Err(Self::Error {
                invalid_flag_state: value,
            }),
            0b1000 => Ok(MoveFlag::KnightPromotion),
            0b1001 => Ok(MoveFlag::BishopPromotion),
            0b1010 => Ok(MoveFlag::RookPromotion),
            0b1011 => Ok(MoveFlag::QueenPromotion),
            0b1100 => Ok(MoveFlag::KnightCapturePromotion),
            0b1101 => Ok(MoveFlag::BishopCapturePromotion),
            0b1110 => Ok(MoveFlag::RookCapturePromotion),
            0b1111 => Ok(MoveFlag::QueenCapturePromotion),
            _ => Err(Self::Error {
                invalid_flag_state: value,
            }),
        }
    }
}

#[derive(Debug)]
pub struct InvalidMoveFlag {
    invalid_flag_state: u8,
}

impl Display for InvalidMoveFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Invalid movement state found: {}",
            self.invalid_flag_state
        )
    }
}
