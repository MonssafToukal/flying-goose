use std::vec;

use crate::board::types::SquareCoord;
use crate::types::SQUARE_MASKS;
use crate::{
    board::types::{EMPTY_BITBOARD, Files, Ranks},
    types::{BitBoard, NumOf},
};

use super::magic::MagicEntry;

pub const ROOK_BLOCKERS_MASK: &[BitBoard; NumOf::SQUARES] = &generate_rook_blockers_masks();
// pub const ROOK_MAGIC_ENTRIES: &[MagicEntry; NumOf::SQUARES] = todo!();

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


pub fn find_magics(square: SquareCoord) -> Vec<MagicEntry> {
    let mut magic_entries: Vec<MagicEntry> = Vec::new();
    // All blockers configurations for that mask
    let blocker_configurations = get_all_blockers_subsets(ROOK_BLOCKERS_MASK[square.to_usize()]);
    
    // For each blocker configuration, find the eligible rook moves bitboard associated to it.
    // There are many ways to do that, one way would be to simply check each square in the direction that a rook moves
    // and stop when you find an obstacle.
    // Consider the blockers as being an opponent's piece such that the blocker could be captured by default.

    // Once the moves are found for each blocker configurations
    // We can try generating magics for each of these blockers

    todo!()
}

pub fn get_all_blockers_subsets(blocker_mask: BitBoard) -> Vec<BitBoard> {
    let mut subsets: Vec<BitBoard> = Vec::new();
    subsets.push(EMPTY_BITBOARD);
    while let Some(current_subset) = subsets.last() && *current_subset != blocker_mask { 
        let next_subset: BitBoard = current_subset.wrapping_sub(blocker_mask) & blocker_mask;
        subsets.push(next_subset);
    }
    subsets
}

