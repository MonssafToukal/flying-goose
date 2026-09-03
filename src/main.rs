#![allow(warnings)]
pub mod board;
pub mod movement;
pub mod types;

use crate::board::types::SquareCoord;
use board::{
    Board,
    history::GameHistory,
    piece_movement::{Move, MoveFlag},
    state::GameState,
    types::{BySide, BySquare, Pieces, Side, Square},
    zobrist::Zobrist,
};
use movement::{
    MovementData, MovementDataInitError,
    sliders::{
        defs::{BISHOP_SLIDER, ROOK_SLIDER, get_all_blockers_subsets},
        magic_entries::{BISHOP_MAGICS, ROOK_MAGICS},
    },
};
use types::{BitBoard, NumOf, SQUARE_MASKS};
use types::{EMPTY_BITBOARD, FILE_MASKS, RANK_MASKS};

fn bb_to_rows(bb: BitBoard) -> Vec<String> {
    const LAST: u64 = 63;
    let mut rows = Vec::with_capacity(10);
    for rank in 0..8u64 {
        let mut row = format!("{}  ", NumOf::RANKS as u64 - rank);
        for file in (0..8u64).rev() {
            let mask = 1u64 << (LAST - rank * 8 - file);
            row.push_str(if mask & bb != 0 { "1 " } else { "0 " });
        }
        rows.push(row);
    }
    rows.push(String::new());
    rows.push("   a b c d e f g h".to_string());
    rows
}

fn print_boards_side_by_side(labels: &[&str], boards: &[BitBoard]) {
    const COL_WIDTH: usize = 24;
    for label in labels {
        print!("{:<COL_WIDTH$}", label);
    }
    println!();
    let rows_list: Vec<Vec<String>> = boards.iter().map(|&bb| bb_to_rows(bb)).collect();
    for row_idx in 0..rows_list[0].len() {
        for rows in &rows_list {
            print!("{:<COL_WIDTH$}", rows[row_idx]);
        }
        println!();
    }
}

/// Builds an empty board (all fields zeroed/default), the same way the
/// #[cfg(test)] module in piece_movement.rs does it, since `Board::new()` is
/// private outside the `board` module and there's no other public
/// zero-arg constructor.
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

fn start_pos() -> Board {
    let mut board = empty_board();
    board.fen_setup(None).unwrap();
    board
}

/// Prints white pieces, black pieces, and all pieces combined side by side
/// so you can visually confirm a `make()` call did what you expect.
fn print_board_state(label: &str, board: &Board) {
    println!("=== {label} ===");
    let mut white_all = EMPTY_BITBOARD;
    let mut black_all = EMPTY_BITBOARD;
    for piece in 0..NumOf::PIECE_TYPES {
        white_all |= board.bb_pieces[Side::White][piece];
        black_all |= board.bb_pieces[Side::Black][piece];
    }
    print_boards_side_by_side(
        &["White pieces", "Black pieces", "All pieces"],
        &[white_all, black_all, white_all | black_all],
    );
    println!(
        "active_color={:?}  castling={:#06b}  enpassant={:?}  half_move_clock={}  fullmove_counter={}  zobrist_key={:#x}",
        board.game_state.active_color,
        board.game_state.castling,
        board.game_state.enpassant,
        board.game_state.half_move_clock,
        board.game_state.fullmove_counter,
        board.game_state.zobrist_key,
    );
    println!();
}

fn main() -> Result<(), MovementDataInitError> {
    // --- Example 1: quiet pawn push -----------------------------------
    let mut board = start_pos();
    print_board_state("Start position", &board);

    board.make(Move::new(Square::E2, Square::E3, MoveFlag::Quiet as u8));
    print_board_state("After 1. e3 (quiet pawn push)", &board);

    // --- Example 2: double pawn push sets en passant -------------------
    let mut board = start_pos();
    board.make(Move::new(Square::E2, Square::E4, MoveFlag::DoublePawnPush as u8));
    print_board_state("After 1. e4 (double pawn push, en passant square set)", &board);

    // --- Example 3: en passant capture ----------------------------------
    // Position after 1.e4 e6 2.e5 d5, set up directly via FEN (rather than
    // replaying moves) so fen_setup()/init() do all the state-rebuilding work.
    let mut board = empty_board();
    board
        .fen_setup(Some(
            "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
        ))
        .unwrap();
    print_board_state("Before en passant capture (after 1.e4 e6 2.e5 d5)", &board);
    board.make(Move::new(Square::E5, Square::D6, MoveFlag::EpCapture as u8));
    print_board_state("After 3. exd6 e.p.", &board);

    // --- Example 4: white king-side castling ----------------------------
    let mut board = empty_board();
    board
        .fen_setup(Some("rnbqkbnr/pppppppp/8/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 3"))
        .unwrap();
    print_board_state("Before white king-side castle", &board);
    board.make(Move::new(Square::E1, Square::G1, MoveFlag::KingSideCastle as u8));
    print_board_state("After O-O (king and rook both moved)", &board);

    // --- Example 5: capture -----------------------------------------
    let mut board = empty_board();
    board
        .fen_setup(Some(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 2",
        ))
        .unwrap();
    print_board_state("Before capture (1.e4 d5)", &board);
    board.make(Move::new(Square::E4, Square::D5, MoveFlag::Capture as u8));
    print_board_state("After 2. exd5 (capture)", &board);

    Ok(())
}
