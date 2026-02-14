use crate::{
    board::types::{EMPTY_BITBOARD, Files, Ranks, SquareCoord},
    movement::sliders::Slider,
    types::{BitBoard, NumOf, SQUARE_MASKS, print_bb},
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct MagicEntry {
    number: u64,
    blocker_masks: BitBoard,
    shift: u8,
}

impl MagicEntry {
    fn new(blocker_masks: BitBoard) -> Self {
        let mut number_of_bits_set = 0;
        let mut temp_blocker = blocker_masks;
        while blocker_masks != 0 {
            number_of_bits_set += 1;
            temp_blocker &= temp_blocker - 1;
        }
        let mut rng = Pcg64::seed_from_u64(42069);
        let num_squares: u8 = u8::try_from(NumOf::SQUARES).unwrap();
        Self {
            number: rng.random(),
            blocker_masks: blocker_masks,
            shift: num_squares - number_of_bits_set,
        }
    }
}

pub fn generate_slider_blockers_masks(slider_piece: &Slider) -> [BitBoard; NumOf::SQUARES] {
    let mut slider_blockers_masks: [BitBoard; NumOf::SQUARES] = [EMPTY_BITBOARD; NumOf::SQUARES];
    for (square_idx, blocker_mask) in slider_blockers_masks.iter_mut().enumerate() {
        let current_square = SquareCoord::try_from(square_idx as u8).unwrap();
        *blocker_mask = slider_piece.get_blockers(current_square);
        print_bb(*blocker_mask);
    }
    slider_blockers_masks
}

pub fn find_magics(square: SquareCoord) -> Vec<MagicEntry> {
    // let mut magic_entries: Vec<MagicEntry> = Vec::new();
    // // All blockers configurations for that mask
    // let blocker_configurations = get_all_blockers_subsets(ROOK_BLOCKERS_MASK[square.to_usize()]);

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
    while let Some(current_subset) = subsets.last()
        && *current_subset != blocker_mask
    {
        let next_subset: BitBoard = current_subset.wrapping_sub(blocker_mask) & blocker_mask;
        subsets.push(next_subset);
    }
    subsets
}
