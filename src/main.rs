#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{Board, fen::FenError, types::{Files, Ranks, Sides, SquareCoord}};
use movement::sliders::generate_rook_occupancy_masks;
use types::print_bb;

fn main() -> Result<(), FenError> {
    let rook_occupancy_mask = generate_rook_occupancy_masks();
    let square = SquareCoord{
        file: Files::D,
        rank: Ranks::R4,
    };
    print_bb(rook_occupancy_mask[square.to_usize()]);

    Ok(())
}
