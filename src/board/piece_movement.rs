use std::fmt::Display;

use crate::{
    board::{
        Board,
        types::{Piece, Pieces, Side, Square},
    },
    types::{NumOf, SQUARE_MASKS},
};

use super::types::SQ;

impl Board {
    // TODO: remaining fixes for make()
    // - Revoke castling rights when a king moves, a rook moves from its home
    //   square, castling happens, or a rook is captured on its home square.
    // - Update half_move_clock (reset on pawn move/capture, increment
    //   otherwise) and fullmove_counter (increment after Black's move).
    // - XOR the zobrist side-to-move hash in toggle_side() so the key
    //   reflects whose turn it is.
    pub fn make(&mut self, chess_move: Move) -> () {
        let mut prev_game_state = self.game_state;
        let from_square: Square = chess_move.from_square();
        let dest_square: Square = chess_move.dest_square();
        let moved_piece: Piece = self.piece_list[from_square];
        // TODO: handle this better than with unwrap
        let move_flags = chess_move.flags().unwrap();

        // compute captured_piece
        match move_flags {
            MoveFlag::Capture
            | MoveFlag::KnightCapturePromotion
            | MoveFlag::BishopCapturePromotion
            | MoveFlag::RookCapturePromotion
            | MoveFlag::QueenCapturePromotion => {
                let captured_piece: Piece = self.piece_list[dest_square];
                let captured_piece_color: Side = self.get_opponent();
                prev_game_state.captured_piece = Some(captured_piece);
                self.remove_piece(captured_piece, captured_piece_color, dest_square);
            }
            MoveFlag::EpCapture => {
                prev_game_state.captured_piece = Some(Pieces::PAWN);
                let (captured_pawn_square, captured_pawn_color): (Square, Side) =
                    match self.game_state.active_color {
                        Side::White => (dest_square.south(), Side::Black),
                        Side::Black => (dest_square.north(), Side::White),
                    };
                self.remove_piece(Pieces::PAWN, captured_pawn_color, captured_pawn_square);
            }
            _ => {}
        }

        // Saving previous state now:
        self.history.push(prev_game_state);

        // Universal move of the moving piece applied here:
        self.move_piece(
            moved_piece,
            self.game_state.active_color,
            from_square,
            dest_square,
        );
        // XOR the zobrist hash from enpassant square if it exists:
        if let Some(enpassant_square) = self.game_state.enpassant {
            self.set_enpassant_move(enpassant_square);
        }

        // Promotions
        match move_flags {
            MoveFlag::KnightPromotion | MoveFlag::KnightCapturePromotion => {
                self.remove_piece(Pieces::PAWN, self.game_state.active_color, dest_square);
                self.put_piece(Pieces::KNIGHT, self.game_state.active_color, dest_square);
            }
            MoveFlag::BishopPromotion | MoveFlag::BishopCapturePromotion => {
                self.remove_piece(Pieces::PAWN, self.game_state.active_color, dest_square);
                self.put_piece(Pieces::BISHOP, self.game_state.active_color, dest_square);
            }
            MoveFlag::RookPromotion | MoveFlag::RookCapturePromotion => {
                self.remove_piece(Pieces::PAWN, self.game_state.active_color, dest_square);
                self.put_piece(Pieces::ROOK, self.game_state.active_color, dest_square);
            }
            MoveFlag::QueenPromotion | MoveFlag::QueenCapturePromotion => {
                self.remove_piece(Pieces::PAWN, self.game_state.active_color, dest_square);
                self.put_piece(Pieces::QUEEN, self.game_state.active_color, dest_square);
            }
            _ => {}
        }

        // DoublePawnPush
        match move_flags {
            MoveFlag::DoublePawnPush => {
                let mut enpassant_square: Square = dest_square;
                if self.game_state.active_color as Side == Side::White {
                    enpassant_square = enpassant_square.south();
                }
                if self.game_state.active_color as Side == Side::Black {
                    enpassant_square = enpassant_square.north();
                }
                self.game_state.set_enpassant(enpassant_square);
                self.set_enpassant_move(enpassant_square);
            }
            _ => {
                self.game_state.clear_enpassant();
            }
        }

        // Castling:

        match move_flags {
            MoveFlag::KingSideCastle => match self.game_state.active_color {
                Side::White => {
                    self.move_piece(
                        Pieces::ROOK,
                        self.game_state.active_color,
                        SQ::H1,
                        dest_square.west(),
                    );
                }
                Side::Black => {
                    self.move_piece(
                        Pieces::ROOK,
                        self.game_state.active_color,
                        SQ::H8 as Square,
                        dest_square.west(),
                    );
                }
            },
            MoveFlag::QueenSideCastle => match self.game_state.active_color {
                Side::White => {
                    self.move_piece(
                        Pieces::ROOK,
                        self.game_state.active_color,
                        SQ::A1 as Square,
                        dest_square.east(),
                    );
                }
                Side::Black => {
                    self.move_piece(
                        Pieces::ROOK,
                        self.game_state.active_color,
                        SQ::A8 as Square,
                        dest_square.east(),
                    );
                }
            },
            _ => {}
        }
        self.game_state.toggle_side();
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
        let file = square.file();
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
            | (dest_square.usize() << Self::DEST_SQUARE_BIT_SHIFT) as u16
            | (flags << Self::FLAGS_BIT_SHIFT) as u16;
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
