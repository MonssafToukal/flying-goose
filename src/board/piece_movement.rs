use crate::{
    board::{
        Board,
        types::{Piece, Square},
    },
    types::NumOf,
};
use std::fmt::Display;

use super::types::{BySquare, CastlingRight, Pieces, Side};

const CASTLING_PERMISSIONS: BySquare<u8> = get_castling_permissions();

const fn get_castling_permissions() -> BySquare<u8> {
    let default_castling_permissions = CastlingRight::ALL;
    let mut permissions = [default_castling_permissions; NumOf::SQUARES];
    // TODO!
    permissions[Square::A1 as usize] &= !(CastlingRight::WhiteQueenSide as u8);
    permissions[Square::E1 as usize] &=
        !(CastlingRight::WhiteQueenSide as u8 | CastlingRight::WhiteKingSide as u8);
    permissions[Square::H1 as usize] &= !(CastlingRight::WhiteKingSide as u8);
    permissions[Square::A8 as usize] &= !(CastlingRight::BlackQueenSide as u8);
    permissions[Square::E8 as usize] &=
        !(CastlingRight::BlackQueenSide as u8 | CastlingRight::BlackKingSide as u8);
    permissions[Square::H8 as usize] &= !(CastlingRight::BlackKingSide as u8);

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
        let captured: Piece = self.piece_list[to_square];
        // TODO: handle this better than with unwrap
        let move_flags = chess_move.flags().unwrap();

        let current_player = self.get_current_player();
        let opponent = self.get_opponent();


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
        if is_captured && !is_enpassant {
            prev_game_state.captured_piece = Some(captured);
            self.game_state.half_move_clock = 0;
            self.remove_piece(captured, opponent, to_square);
        }

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
                prev_game_state.captured_piece = Some(Pieces::PAWN);
                let opponent_pawn_direction = Board::get_pawn_direction(opponent);
                let opponent_pawn_square = (to_square.usize() as i16) + opponent_pawn_direction as i16;
                let opponent_pawn_square = Square::from(opponent_pawn_square as usize);
                self.remove_piece(Pieces::PAWN, opponent, opponent_pawn_square);
                self.set_enpassant(to_square);
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

        self.history.push(prev_game_state);

        // Handle Castling
        let new_castling_permissions =
            CASTLING_PERMISSIONS[from_square] & CASTLING_PERMISSIONS[to_square];
        self.update_castling_permissions(new_castling_permissions);

        if is_castling {
            match to_square {
                Square::G1 => self.move_piece(Pieces::ROOK, current_player, Square::H1, Square::F1),
                Square::C1 => self.move_piece(Pieces::ROOK, current_player, Square::A1, Square::D1),
                Square::G8 => self.move_piece(Pieces::ROOK, current_player, Square::H8, Square::F8),
                Square::C8 => self.move_piece(Pieces::ROOK, current_player, Square::A8, Square::D8),
                _ => panic!("error in make: moving rook during castling matched an impossible square {:?}", to_square),
            }
        }


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

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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
        let dest_square = (self.0 & Self::DEST_SQUARE_MASK) >> (Self::DEST_SQUARE_BIT_SHIFT);
        Square::from(dest_square)
    }

    pub fn flags(&self) -> Result<MoveFlag, InvalidMoveFlag> {
        let move_flag_values = ((self.0 & Self::FLAGS_MASK)  >> Self::FLAGS_BIT_SHIFT) as u8;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::history::GameHistory;
    use crate::board::state::GameState;
    use crate::board::types::{BySide, BySquare, CastlingRight};
    use crate::board::zobrist::Zobrist;
    use crate::types::EMPTY_BITBOARD;

    fn empty_board() -> Board {
        Board {
            bb_pieces: BySide::new([EMPTY_BITBOARD; NumOf::PIECE_TYPES]),
            bb_sides: BySide::new(EMPTY_BITBOARD),
            piece_list: BySquare::new(Pieces::NONE),
            game_state: GameState::new(),
            history: GameHistory::new(),
            zobrist_hashmap: Zobrist::new(None),
        }
    }

    fn board_from_fen(fen: &str) -> Board {
        let mut board = empty_board();
        board.fen_setup(Some(fen)).unwrap();
        board
    }

    fn start_pos() -> Board {
        let mut board = empty_board();
        board.fen_setup(None).unwrap();
        board
    }

    #[test]
    fn quiet_pawn_push_moves_piece_and_toggles_side() {
        let mut board = start_pos();
        let mv = Move::new(Square::E2, Square::E3, MoveFlag::Quiet as u8);

        board.make(mv);

        assert_eq!(board.piece_list[Square::E2], Pieces::NONE);
        assert_eq!(board.piece_list[Square::E3], Pieces::PAWN);
        assert_eq!(board.game_state.active_color, Side::Black);
        assert_eq!(board.game_state.half_move_clock, 0);
        assert_eq!(board.game_state.enpassant, None);
    }

    #[test]
    fn double_pawn_push_sets_enpassant_square() {
        let mut board = start_pos();
        let mv = Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8);

        board.make(mv);

        assert_eq!(board.piece_list[Square::E2], Pieces::NONE);
        assert_eq!(board.piece_list[Square::E4], Pieces::PAWN);
        // White just moved, so the ep-capturable square is behind the pawn: e3.
        assert_eq!(board.game_state.enpassant, Some(Square::E3));
    }

    #[test]
    fn second_quiet_move_clears_previous_enpassant_square() {
        // After 1. e4, en passant is set on e3. A following unrelated quiet
        // move must clear it.
        let mut board = start_pos();
        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.game_state.enpassant, Some(Square::E3));

        board.make(Move::new(Square::B8, Square::C6, MoveFlag::Quiet as u8));

        assert_eq!(board.game_state.enpassant, None);
    }

    #[test]
    fn enpassant_capture_removes_captured_pawn_and_records_it() {
        // 1. e4 e6 2. e5 d5 3. exd6 e.p.
        let mut board = start_pos();
        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));
        board.make(Move::new(Square::E7, Square::E6, MoveFlag::Quiet as u8));
        board.make(Move::new(Square::E4, Square::E5, MoveFlag::Quiet as u8));
        board.make(Move::new(Square::D7, Square::D5, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.game_state.enpassant, Some(Square::D6));

        board.make(Move::new(Square::E5, Square::D6, MoveFlag::EpCapture as u8));

        // Capturing pawn now on d6, black pawn that was on d5 is gone.
        assert_eq!(board.piece_list[Square::D6], Pieces::PAWN);
        assert_eq!(board.piece_list[Square::D5], Pieces::NONE);
        assert_eq!(board.piece_list[Square::E5], Pieces::NONE);
        // The captured piece recorded for unmake/history should be a pawn,
        // not Pieces::NONE (there is nothing sitting on the destination
        // square d6 before an en-passant capture).
        let prev_state = board.history.get_last().expect("history should have an entry");
        assert_eq!(prev_state.captured_piece, Some(Pieces::PAWN));
    }

    #[test]
    fn black_enpassant_capture_removes_captured_pawn_and_records_it() {
        // Black pawn already on d4. White plays e2-e4 (double push, sets ep
        // square e3), then Black replies d4xe3 e.p.
        let fen = "rnbqkbnr/ppp1pppp/8/8/3p4/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let mut board = board_from_fen(fen);

        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.game_state.enpassant, Some(Square::E3));

        board.make(Move::new(Square::D4, Square::E3, MoveFlag::EpCapture as u8));

        // Capturing pawn now on e3, white pawn that was on e4 is gone.
        assert_eq!(board.piece_list[Square::E3], Pieces::PAWN);
        assert_eq!(board.piece_list[Square::D4], Pieces::NONE);
        assert_eq!(board.piece_list[Square::E4], Pieces::NONE);
        let prev_state = board.history.get_last().expect("history should have an entry");
        assert_eq!(prev_state.captured_piece, Some(Pieces::PAWN));
    }

    #[test]
    fn capture_records_captured_piece_and_resets_halfmove_clock() {
        let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2";
        let mut board = board_from_fen(fen);
        board.game_state.half_move_clock = 7;

        let mv = Move::new(Square::E4, Square::D5, MoveFlag::Capture as u8);
        board.make(mv);

        assert_eq!(board.piece_list[Square::D5], Pieces::PAWN);
        assert_eq!(board.piece_list[Square::E4], Pieces::NONE);
        assert_eq!(board.game_state.half_move_clock, 0);
        let prev_state = board.history.get_last().expect("history should have an entry");
        assert_eq!(prev_state.captured_piece, Some(Pieces::PAWN));
    }

    #[test]
    fn promotion_replaces_pawn_with_promoted_piece() {
        let fen = "rnbqkbnr/ppp1pP1p/8/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 6";
        let mut board = board_from_fen(fen);

        let mv = Move::new(Square::F7, Square::F8, MoveFlag::QueenPromotion as u8);
        board.make(mv);

        assert_eq!(board.piece_list[Square::F8], Pieces::QUEEN);
        assert_eq!(board.piece_list[Square::F7], Pieces::NONE);
    }

    #[test]
    fn white_kingside_castle_moves_both_king_and_rook() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 3";
        let mut board = board_from_fen(fen);

        let mv = Move::new(Square::E1, Square::G1, MoveFlag::KingSideCastle as u8);
        board.make(mv);

        assert_eq!(board.piece_list[Square::G1], Pieces::KING);
        assert_eq!(board.piece_list[Square::F1], Pieces::ROOK);
        assert_eq!(board.piece_list[Square::E1], Pieces::NONE);
        assert_eq!(board.piece_list[Square::H1], Pieces::NONE);
    }

    #[test]
    fn black_queenside_castle_moves_both_king_and_rook() {
        let fen = "r3kbnr/pppqpppp/2n5/3p1b2/3P1B2/2N5/PPPQPPPP/R3KBNR b KQkq - 6 5";
        let mut board = board_from_fen(fen);

        let mv = Move::new(Square::E8, Square::C8, MoveFlag::QueenSideCastle as u8);
        board.make(mv);

        assert_eq!(board.piece_list[Square::C8], Pieces::KING);
        assert_eq!(board.piece_list[Square::D8], Pieces::ROOK);
        assert_eq!(board.piece_list[Square::E8], Pieces::NONE);
        assert_eq!(board.piece_list[Square::A8], Pieces::NONE);
    }

    #[test]
    fn king_move_revokes_both_castling_rights_for_that_side() {
        let mut board = start_pos();
        let all_rights = CastlingRight::ALL;
        assert_eq!(board.game_state.castling, all_rights);

        // Clear the squares between king and knight isn't necessary: we only
        // need to test permission bookkeeping, so move the king directly
        // (illegal in a real game since e2 pawn hasn't moved, but make() does
        // not check legality yet).
        let mv = Move::new(Square::E1, Square::E2, MoveFlag::Quiet as u8);
        board.make(mv);

        let white_king_side = CastlingRight::WhiteKingSide as u8;
        let white_queen_side = CastlingRight::WhiteQueenSide as u8;
        let black_king_side = CastlingRight::BlackKingSide as u8;
        let black_queen_side = CastlingRight::BlackQueenSide as u8;

        assert_eq!(board.game_state.castling & white_king_side, 0);
        assert_eq!(board.game_state.castling & white_queen_side, 0);
        // Black's rights must be untouched.
        assert_eq!(board.game_state.castling & black_king_side, black_king_side);
        assert_eq!(board.game_state.castling & black_queen_side, black_queen_side);
    }

    #[test]
    fn rook_move_revokes_only_that_sides_castling_right() {
        let mut board = start_pos();

        // Move the a1 rook (through an empty square is not required since
        // make() does not validate legality/path).
        let mv = Move::new(Square::A1, Square::A2, MoveFlag::Quiet as u8);
        board.make(mv);

        let white_king_side = CastlingRight::WhiteKingSide as u8;
        let white_queen_side = CastlingRight::WhiteQueenSide as u8;

        assert_eq!(board.game_state.castling & white_queen_side, 0);
        assert_eq!(
            board.game_state.castling & white_king_side,
            white_king_side,
            "moving the a1 rook must not revoke king-side rights"
        );
    }

    #[test]
    fn unrelated_quiet_move_does_not_restore_lost_castling_rights() {
        // Regression test: update_castling_permissions must only ever shrink
        // rights (AND), never restore ones already lost.
        let mut board = start_pos();
        board.make(Move::new(Square::A1, Square::A2, MoveFlag::Quiet as u8)); // white
        let white_queen_side = CastlingRight::WhiteQueenSide as u8;
        assert_eq!(board.game_state.castling & white_queen_side, 0);

        // Black plays an unrelated move so turn order stays valid.
        board.make(Move::new(Square::B8, Square::C6, MoveFlag::Quiet as u8)); // black

        // An unrelated white knight move (neither endpoint square carries any
        // castling-permission restriction) must not resurrect white's
        // already-revoked queen-side right.
        board.make(Move::new(Square::G1, Square::F3, MoveFlag::Quiet as u8)); // white

        assert_eq!(
            board.game_state.castling & white_queen_side,
            0,
            "castling rights must never be restored once revoked"
        );
    }

    #[test]
    fn fullmove_counter_increments_only_after_black_moves() {
        let mut board = start_pos();
        assert_eq!(board.game_state.fullmove_counter, 1);

        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.game_state.fullmove_counter, 1);

        board.make(Move::new(Square::E7, Square::E5, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.game_state.fullmove_counter, 2);
    }

    #[test]
    fn half_move_clock_increments_on_quiet_non_pawn_move() {
        let fen = "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 1 1";
        let mut board = board_from_fen(fen);
        assert_eq!(board.game_state.half_move_clock, 1);

        board.make(Move::new(Square::B8, Square::C6, MoveFlag::Quiet as u8));

        assert_eq!(board.game_state.half_move_clock, 2);
    }

    #[test]
    fn active_color_toggles_each_move() {
        let mut board = start_pos();
        assert_eq!(board.game_state.active_color, Side::White);

        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.game_state.active_color, Side::Black);

        board.make(Move::new(Square::E7, Square::E5, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.game_state.active_color, Side::White);
    }

    #[test]
    fn quiet_move_leaves_no_captured_piece_in_history() {
        let mut board = start_pos();
        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));

        let prev_state = board.history.get_last().expect("history should have an entry");
        assert_eq!(prev_state.captured_piece, None);
    }

    #[test]
    fn history_grows_by_one_per_move_and_preserves_order() {
        let mut board = start_pos();
        assert_eq!(board.history.len(), 0);

        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.history.len(), 1);
        // The very first history entry must reflect the state BEFORE the
        // first move: white to move, full castling rights, no en passant.
        let first_entry = board.history.get(0);
        assert_eq!(first_entry.active_color, Side::White);
        assert_eq!(first_entry.castling, CastlingRight::ALL);
        assert_eq!(first_entry.enpassant, None);
        assert_eq!(first_entry.fullmove_counter, 1);

        board.make(Move::new(Square::E7, Square::E5, MoveFlag::DoublePawnPush as u8));
        assert_eq!(board.history.len(), 2);
        // Second history entry captures the state after white's move: black
        // to move, en passant on e3 still set (about to be cleared by this
        // very move).
        let second_entry = board.history.get(1);
        assert_eq!(second_entry.active_color, Side::Black);
        assert_eq!(second_entry.enpassant, Some(Square::E3));
    }

    #[test]
    fn history_records_the_move_that_was_made() {
        let mut board = start_pos();
        let mv = Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8);
        board.make(mv);

        let prev_state = board.history.get_last().expect("history should have an entry");
        assert_eq!(prev_state.next_move, mv);
    }

    #[test]
    fn zobrist_key_changes_after_a_move() {
        let mut board = start_pos();
        let key_before = board.game_state.zobrist_key;

        board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));

        assert_ne!(
            board.game_state.zobrist_key, key_before,
            "zobrist key must change after a move that alters piece placement, side, and en passant state"
        );
    }

    #[test]
    fn zobrist_key_toggles_back_after_two_moves_returning_to_similar_state() {
        // Two knight moves out and back (Nf3, Nf6) return the pieces to their
        // starting squares and side to White again, but the zobrist key
        // should NOT necessarily match the start position's key, since the
        // half-move clock / history differ conceptually. However it SHOULD
        // at least differ from the key immediately after the first move,
        // proving the key is sensitive to more than just "whose turn".
        let mut board = start_pos();
        let key_after_ply1_setup = board.game_state.zobrist_key;

        board.make(Move::new(Square::G1, Square::F3, MoveFlag::Quiet as u8));
        let key_after_ply1 = board.game_state.zobrist_key;
        assert_ne!(key_after_ply1_setup, key_after_ply1);

        board.make(Move::new(Square::G8, Square::F6, MoveFlag::Quiet as u8));
        let key_after_ply2 = board.game_state.zobrist_key;
        assert_ne!(key_after_ply1, key_after_ply2);
    }

    #[test]
    fn capture_promotion_replaces_pawn_and_removes_captured_piece() {
        // White pawn on b7 can capture on a8 (a black rook) and promote.
        let fen = "rnbqkbnr/pP2pppp/8/8/8/8/P1PPPPPP/RNBQKBNR w KQkq - 0 5";
        let mut board = board_from_fen(fen);

        let mv = Move::new(Square::B7, Square::A8, MoveFlag::QueenCapturePromotion as u8);
        board.make(mv);

        assert_eq!(board.piece_list[Square::A8], Pieces::QUEEN);
        assert_eq!(board.piece_list[Square::B7], Pieces::NONE);
        let prev_state = board.history.get_last().expect("history should have an entry");
        assert_eq!(prev_state.captured_piece, Some(Pieces::ROOK));
    }

    #[test]
    fn capture_promotion_revokes_castling_right_of_captured_rook_square() {
        // Capturing black's a8 rook via promotion must revoke black's
        // queen-side castling right, since CASTLING_PERMISSIONS keys off
        // to_square regardless of move flag.
        let fen = "rnbqkbnr/pP2pppp/8/8/8/8/P1PPPPPP/RNBQKBNR w KQkq - 0 5";
        let mut board = board_from_fen(fen);

        board.make(Move::new(Square::B7, Square::A8, MoveFlag::QueenCapturePromotion as u8));

        let black_queen_side = CastlingRight::BlackQueenSide as u8;
        assert_eq!(board.game_state.castling & black_queen_side, 0);
    }

    #[test]
    fn capturing_rook_on_its_home_square_revokes_that_sides_castling_right() {
        // White bishop on g6 captures whatever sits on h8; even though
        // black's own rook never moved off h8 in this position, capturing on
        // h8 must revoke black's king-side right since CASTLING_PERMISSIONS
        // keys off to_square, not off which piece is captured.
        let fen = "rnbqkb1r/pppppp1p/6B1/8/8/8/PPPPPPPP/RNBQK1NR w KQkq - 0 3";
        let mut board = board_from_fen(fen);
        let black_king_side = CastlingRight::BlackKingSide as u8;
        assert_eq!(
            board.game_state.castling & black_king_side, black_king_side,
            "sanity check: black should still have king-side rights before the capture"
        );

        board.make(Move::new(Square::G6, Square::H8, MoveFlag::Capture as u8));

        assert_eq!(board.game_state.castling & black_king_side, 0);
    }

    #[test]
    fn king_capturing_rook_revokes_both_of_that_sides_rights_via_from_square() {
        let fen = "rnbqk1nr/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/RNBQKB1b w KQkq - 0 3";
        let mut board = board_from_fen(fen);

        // White king captures the black bishop sitting on h1 (illegal in a
        // real game from e1, but make() does not validate legality).
        let mv = Move::new(Square::E1, Square::H1, MoveFlag::Capture as u8);
        board.make(mv);

        let white_king_side = CastlingRight::WhiteKingSide as u8;
        let white_queen_side = CastlingRight::WhiteQueenSide as u8;
        assert_eq!(board.game_state.castling & white_king_side, 0);
        assert_eq!(board.game_state.castling & white_queen_side, 0);
    }

    #[test]
    fn captured_piece_is_removed_from_bitboards_not_just_piece_list() {
        let fen = "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2";
        let mut board = board_from_fen(fen);

        board.make(Move::new(Square::E4, Square::D5, MoveFlag::Capture as u8));

        // Black no longer has any pawn bit set on d5, and black's aggregate
        // side bitboard must not include d5 either.
        let d5_mask = crate::types::SQUARE_MASKS[Square::D5];
        assert_eq!(board.bb_pieces[Side::Black][Pieces::PAWN] & d5_mask, 0);
        assert_eq!(board.bb_sides[Side::Black] & d5_mask, 0);
        // White's pawn bitboard must now include d5.
        assert_ne!(board.bb_pieces[Side::White][Pieces::PAWN] & d5_mask, 0);
        assert_ne!(board.bb_sides[Side::White] & d5_mask, 0);
    }
}
