#![allow(warnings)]
pub mod board;
pub mod movement;
pub mod types;

use crate::board::types::SquareCoord;
use board::types::{Side, Square};
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

fn main() -> Result<(), MovementDataInitError> {
    let mut movement_data = MovementData::new();
    movement_data.init()?;

    for color in 0..NumOf::SIDES {
        let color = Side::from(color);
        match color {
            Side::White => println!("WHITE PAWNS"),
            Side::Black => println!("BLACK PAWNS"),
        }
        for square_idx in 0..NumOf::SQUARES {
            let square = Square::from(square_idx);
            print_boards_side_by_side(
                &["Pawn position", "Pawn attacks"],
                &[
                    SQUARE_MASKS[square],
                    movement_data.pawn_attacks[color][square_idx],
                ],
            );
            println!();
        }
    }

    Ok(())
}
