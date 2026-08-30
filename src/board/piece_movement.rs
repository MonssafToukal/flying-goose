use std::fmt::Display;

use crate::{
    board::{
        Board, state::GameState, types::{Piece, Pieces, Side, Square}
    },
    types::{NumOf, SQUARE_MASKS},
};

use super::types::{BySquare, CastlingRight};

const CASTLING_PERMISSIONS: BySquare<u8> = get_castling_permissions();

const fn get_castling_permissions() -> BySquare<u8> {
    let default_castling_permissions = CastlingRight::ALL;
    let mut permissions = [default_castling_permissions; NumOf::SQUARES];
    // TODO!
    permissions[Square::A1 as usize] = !(CastlingRight::WhiteQueenSide as u8);
    permissions[Square::E1 as usize] =
        !(CastlingRight::WhiteQueenSide as u8 | CastlingRight::WhiteKingSide as u8);
    permissions[Square::A1 as usize] = !(CastlingRight::WhiteKingSide as u8);
    permissions[Square::A8 as usize] = !(CastlingRight::BlackQueenSide as u8);
    permissions[Square::E8 as usize] =
        !(CastlingRight::BlackQueenSide as u8 | CastlingRight::BlackKingSide as u8);
    permissions[Square::A8 as usize] = !(CastlingRight::BlackKingSide as u8);

    let permissions = BySquare::init(permissions);

    permissions
}

impl Board {
    /*
     * Some considerations:
     *  The move encoding scheme is not great.
     *  The reason is because the move flags encodes if:
     *  a capture occurs but doesn't tell me what piece can be captured.
     *  Another problem is with enpassant.
     *  I have a flag set when when a pawn takes en passant
     *  But I also have a field in game_state that tells me if there is an enpassant square.
     *  One of them are redundant.
     *  Need to rework this
    */
    pub fn make(&mut self, chess_move: Move) -> () {

        //Decomposing the information from Move bitset:
        let from_square: Square = chess_move.from_square();
        let to_square: Square = chess_move.dest_square();
        let moved_piece: Piece = self.piece_list[from_square];
        // TODO: handle this better than with unwrap
        let move_flags = chess_move.flags().unwrap();

        let current_player = self.get_current_player();
        let opponent = self.get_opponent();

        let moved: Piece = self.piece_list[from_square];
        let captured: Piece = self.piece_list[to_square];

        let is_captured = move_flags.is_capture();
        let is_promotion = move_flags.is_promotion();
        let is_enpassant = move_flags.is_enpassant();
        let is_double_pawn_push = move_flags.is_double_pawn_push();

        // Saving the previous gamestate before making moves
        let mut prev_game_state = self.game_state;

        // add ply to ply counter:
        self.game_state.half_move_clock += 1;
        // Saving captured piece if any in the old game_state
        if is_captured {
            prev_game_state.captured_piece = Some(captured);
            self.game_state.half_move_clock = 0;
            self.remove_piece(captured, opponent, to_square);
        }
        self.history.push(prev_game_state);

        // Clear enpassant square first
        self.clear_enpassant();

        // Handle Castling
        let new_castling_permissions = CASTLING_PERMISSIONS[from_square] & CASTLING_PERMISSIONS[to_square];



        todo!()
    }

    // pub fn unmake(&mut self, piece: Piece, from_square: Square, to_square: Square) -> () {
    //     todo!()
    // }
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
        if let Some(ep_square) = self.game_state.enpassant {
            let file = ep_square.file();
            self.game_state.zobrist_key ^= self.zobrist_hashmap.enpassant(file);
        }
        self.game_state.clear_enpassant();
    }

}

#[derive(Clone, Copy)]
pub struct Move(u16);

// TODO: review this encoding as it simply doesn't seem good enough.

impl Move {
    const DEST_SQUARE_BIT_SHIFT: u8 = 6;
    const FLAGS_BIT_SHIFT: u8 = 12;
    const FROM_SQUARE_MASK: u16 = 0x003F;
    const DEST_SQUARE_MASK: u16 = 0x003F << Self::DEST_SQUARE_BIT_SHIFT;
    const FLAGS_MASK: u16 = 0x0F << Self::FLAGS_BIT_SHIFT;

    pub fn new(from_square: Square, dest_square: Square, flags: u8) -> Self {
        let chess_move: u16 = from_square as u16
            | (dest_square.usize() << Self::DEST_SQUARE_BIT_SHIFT) as u16
            | ((flags as u16) << Self::FLAGS_BIT_SHIFT);
        Move(chess_move)
    }
    pub fn from_square(&self) -> Square {
        Square::from(self.0 & Self::FROM_SQUARE_MASK)
    }

    pub fn dest_square(&self) -> Square {
        Square::from(self.0 & Self::DEST_SQUARE_MASK)
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

impl MoveFlag {
    const PROMOTION_MASK: u8 = 0b1000;
    #[inline(always)]
    pub fn is_capture(&self) -> bool {
        (*self as u8) & (Self::Capture as u8) != 0x00
    }

    #[inline(always)]
    pub fn is_promotion(&self) -> bool {
        ((*self as u8) & Self::PROMOTION_MASK) != 0x00
    }

    #[inline(always)]
    pub fn is_enpassant(&self) -> bool {
        *self == Self::EpCapture
    }

    #[inline(always)]
    pub fn is_double_pawn_push(&self) -> bool {
        *self == Self::DoublePawnPush
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
