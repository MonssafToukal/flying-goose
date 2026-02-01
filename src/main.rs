#![allow(warnings)]

pub mod board;
pub mod movement;
pub mod types;

use board::{
    Board,
    fen::FenError,
    types::{Files, Ranks, Sides, SquareCoord},
};
use movement::sliders::ROOK_BLOCKERS_MASK;
use types::{SQUARE_MASKS, print_bb};

fn main() -> Result<(), FenError> {
    // let square = SquareCoord{
    //     file: Files::D,
    //     rank: Ranks::R4,
    // };
    // print_bb(ROOK_OCCUPANCY_MASKS[square.to_usize()]);
    // print_bb(SQUARE_MASKS[square.to_usize()]);

    ROOK_BLOCKERS_MASK.iter().for_each(|bb| {
        print_bb(*bb);
    });

    Ok(())
}
