use crate::{
    board::{
        Board,
        types::{Piece, Square},
    },
    types::NumOf,
};
use std::fmt::Display;

use super::types::{BySquare, CastlingRight, Pieces};

const CASTLING_PERMISSIONS: BySquare<u8> = get_castling_permissions();

const fn get_castling_permissions() -> BySquare<u8> {
    let default_castling_permissions = CastlingRight::ALL;
    let mut permissions = [default_castling_permissions; NumOf::SQUARES];
    // TODO!
    permissions[Square::A1 as usize] = !(CastlingRight::WhiteQueenSide as u8);
    permissions[Square::E1 as usize] =
        !(CastlingRight::WhiteQueenSide as u8 | CastlingRight::WhiteKingSide as u8);
    permissions[Square::H1 as usize] = !(CastlingRight::WhiteKingSide as u8);
    permissions[Square::A8 as usize] = !(CastlingRight::BlackQueenSide as u8);
    permissions[Square::E8 as usize] =
        !(CastlingRight::BlackQueenSide as u8 | CastlingRight::BlackKingSide as u8);
    permissions[Square::H8 as usize] = !(CastlingRight::BlackKingSide as u8);

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

        let is_castling = move_flags.is_castling();
        let is_captured = move_flags.is_capture();
        let is_promotion = move_flags.is_promotion();
        let is_enpassant = move_flags.is_enpassant();
        let is_double_pawn_push = move_flags.is_double_pawn_push();

        // Saving the previous gamestate before making moves
        let mut prev_game_state = self.game_state;
        prev_game_state.next_move = chess_move;

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

        if moved_piece == Pieces::PAWN {
            self.remove_piece(moved_piece, current_player, from_square);
            let piece_to_put: Piece = match move_flags {
                MoveFlag::KnightPromotion => Pieces::KNIGHT,
                MoveFlag::BishopPromotion => Pieces::BISHOP,
                MoveFlag::RookPromotion => Pieces::ROOK,
                MoveFlag::QueenPromotion => Pieces::QUEEN,
                MoveFlag::KnightCapturePromotion => Pieces::KNIGHT,
                MoveFlag::BishopCapturePromotion => Pieces::BISHOP,
                MoveFlag::RookCapturePromotion => Pieces::ROOK,
                MoveFlag::QueenCapturePromotion => Pieces::QUEEN,
                _ => Pieces::PAWN,
            };
            self.put_piece(piece_to_put, current_player, to_square);
            self.game_state.half_move_clock = 0;

            if is_enpassant {
                // remove the pawn on the enpassant_square:
                /*
                1. find the opponents pawn direction (+8 or -8) for white and black respectively
                2. from this offset, calculate the square position of the pawn that was captured en passant
                3. remove it
                 */
                let opponent_pawn_direction = Board::get_pawn_direction(opponent);
                let opponent_pawn_square = (to_square.usize() as i16) + opponent_pawn_direction as i16;
                let opponent_pawn_square = Square::from(opponent_pawn_square as usize);
                self.remove_piece(Pieces::PAWN, opponent, opponent_pawn_square);
            }

            if is_double_pawn_push {
                let opponent_pawn_direction = Board::get_pawn_direction(opponent);
                let ep_square = (to_square.usize() as i16) + opponent_pawn_direction as i16;
                let ep_square = Square::from(ep_square as usize);
                self.set_enpassant(ep_square);
            }
        } else {
            self.move_piece(moved_piece, current_player, from_square, to_square);
        }

        // Handle Castling
        let new_castling_permissions =
            CASTLING_PERMISSIONS[from_square] & CASTLING_PERMISSIONS[to_square];
        self.update_castling_permissions(new_castling_permissions);

        if current_player == Side::Black {
            self.game_state.fullmove_counter += 1;
        }
        self.toggle_side();

        // TODO: check if the move is legal and return an error if not.
    }

    // pub fn unmake(&mut self, piece: Piece, from_square: Square, to_square: Square) -> () {
    //     todo!()
    // }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Ord)]
pub struct Move(u16);

// TODO: review this encoding as it simply doesn't seem good enough.
/*
Current encoding:
0000 0000 0011 1111 from square
0000 1111 1100 0000 to square
1111 0000 0000 0000 flags
 */

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

    #[inline(always)]
    fn is_castling(&self) -> bool {
        match *self {
            MoveFlag::KingSideCastle => true,
            MoveFlag::QueenSideCastle => true,
            _ => false,
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
