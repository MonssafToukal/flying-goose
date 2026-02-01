use std::vec;

use crate::board::types::SquareCoord;
use crate::types::SQUARE_MASKS;
use crate::{
    board::types::{EMPTY_BITBOARD, Files, Ranks},
    types::{BitBoard, NumOf},
};

pub const ROOK_BLOCKERS_MASK: [BitBoard; NumOf::SQUARES] = generate_rook_blockers_masks();

// To make this a const function, I need to hardcode some values unfortunately
const fn generate_rook_blockers_masks() -> [BitBoard; NumOf::SQUARES] {
    let mut rook_blockers_masks: [BitBoard; NumOf::SQUARES] = [EMPTY_BITBOARD; NumOf::SQUARES];
    let nb_files: u8 = 8;
    let nb_ranks: u8 = 8;
    let mut rank: u8 = 0;
    let mut file: u8 = 0;
    while rank < nb_ranks {
        file = 0;
        while file < nb_files {
            let current_square: usize = file as usize + (rank * nb_files) as usize;
            rook_blockers_masks[current_square] = get_blockers_mask(rank, file);
            file += 1;
        }
        rank += 1;
    }
    rook_blockers_masks
}

const fn get_blockers_mask(initial_rank: u8, initial_file: u8) -> BitBoard {
    let nb_files: u8 = 8;
    let mut occupancy_mask = EMPTY_BITBOARD;
    let mut right_file = initial_file + 1;
    // Going in the right direction
    while right_file < Files::H as u8 {
        let current_square_index = right_file + initial_rank * nb_files;
        occupancy_mask |= SQUARE_MASKS[current_square_index as usize];
        right_file += 1;
    }
    // Going in the left direction if not on A file
    if initial_file != 0 {
        let mut left_file = initial_file - 1;
        while left_file > Files::A as u8 {
            let current_square_index = left_file + initial_rank * nb_files;
            occupancy_mask |= SQUARE_MASKS[current_square_index as usize];
            left_file -= 1;
        }
    }
    //Going in the up direction
    let mut up_rank = initial_rank + 1;
    while up_rank < Ranks::R8 as u8 {
        let current_square_index = initial_file + up_rank * nb_files;
        occupancy_mask |= SQUARE_MASKS[current_square_index as usize];
        up_rank += 1;
    }
    if initial_rank != 0 {
        let mut down_rank = initial_rank - 1;
        while down_rank > Ranks::R1 as u8 {
            let current_square_index = initial_file + down_rank * nb_files;
            occupancy_mask |= SQUARE_MASKS[current_square_index as usize];
            down_rank -= 1;
        }
    }
    occupancy_mask
}

pub fn find_all_blockers_subsets() -> Vec<BitBoard> {
    let mut subsets: Vec<u8> = Vec::new();
    let test_bitset: u8 = 0b00100011;
    subsets.push(0u8);
    while let Some(current_subset)  = subsets.last() && *current_subset != test_bitset {
        todo!()
    }
    todo!()
}
