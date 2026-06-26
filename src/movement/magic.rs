use crate::{
    board::types::{EMPTY_BITBOARD, Files, Ranks, SquareCoord},
    movement::sliders::{Slider, get_all_blockers_subsets},
    types::{BitBoard, NumOf, SQUARE_MASKS},
};
use rand::{Rng, SeedableRng, seq::IndexedRandom};
use rand_pcg::Pcg64;

use super::sliders::ROOK_SLIDER;


const RANDOM_SEED : u64 = 42069;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagicEntry {
    number: u64,
    blocker_mask: BitBoard,
    offset: u32,
    index_bits: u8,
    shift: u8,
}

impl MagicEntry {
    pub fn new(blocker_mask: BitBoard) -> Self {
        let mut number_of_bits_set = 0;
        let mut temp_blocker = blocker_mask;
        while temp_blocker != 0 {
            number_of_bits_set += 1;
            temp_blocker &= temp_blocker - 1;
        }
        let mut rng = Pcg64::seed_from_u64(RANDOM_SEED);
        let mut magic_numbers = [0u64, 3];
        rng.fill(&mut magic_numbers);
        let magic_number: u64 = magic_numbers.into_iter().reduce(|acc, m| acc & m).unwrap();
        let num_squares: u8 = u8::try_from(NumOf::SQUARES).unwrap();
        Self {
            number: magic_number,
            blocker_mask: blocker_mask,
            offset: 1 << number_of_bits_set,
            index_bits: number_of_bits_set,
            shift: num_squares - number_of_bits_set,
        }
    }
}

// TODO: Find a way to get this as arrays instead of vectors
fn get_magics(slider: &Slider) -> (Vec<MagicEntry>, Vec<BitBoard>) {
    let total_lookup_table_size: usize = get_total_lookup_table_size(slider);
    let mut lookup_table: Vec<BitBoard> = vec![EMPTY_BITBOARD; total_lookup_table_size];
    let mut magic_entries: Vec<MagicEntry> = Vec::with_capacity(NumOf::SQUARES);
    let mut current_offset = 0u64;
    for square_idx in 0..NumOf::SQUARES {
        let square = SquareCoord::try_from(square_idx as u8).unwrap();
        let (magic_entry, table) = find_magic(slider, square);
        magic_entries.push(magic_entry);
        for (i, table_entry) in table.iter().enumerate() {
            lookup_table[i + current_offset as usize] = *table_entry;
        }
        current_offset += magic_entry.offset as u64;
    }
    (magic_entries, lookup_table)
}

fn get_total_lookup_table_size(slider: &Slider) -> usize {
    let mut total_lookup_table_size:usize = 0;
    let mut blockers = slider.get_all_blockers();
    blockers.iter().for_each(|blocker| {
        let mut counter = 0;
        let mut temp_blocker = *blocker;
        while temp_blocker != EMPTY_BITBOARD {
            counter +=1;
            temp_blocker &= (temp_blocker - 1);
        }
        total_lookup_table_size += (1 << counter);
    });
    total_lookup_table_size
}

fn find_magic(slider: &Slider, square: SquareCoord) -> (MagicEntry, Vec<BitBoard>) {
    let blocker_mask = slider.get_blocker_mask(square);
    loop {
        let mut magic = MagicEntry::new(blocker_mask);
        if let Ok(lookup_table) = get_lookup_table(&slider, &magic, square) {
            return (magic, lookup_table);
        }
    }
}


struct LookupTableCreationError;

fn get_lookup_table(slider: &Slider, magic_entry: &MagicEntry, square: SquareCoord) -> Result<Vec<BitBoard>, LookupTableCreationError> {
    let lookup_table = vec![EMPTY_BITBOARD; 1 << magic_entry.index_bits];
    for blocker_subset in get_all_blockers_subsets(magic_entry.blocker_mask) {
        let eligible_moves = slider.get_moves(square, magic_entry.blocker_mask);
        let index = get_magic_index(&magic_entry, blocker_subset);
        let mut table_entry = lookup_table[index];
        if table_entry == EMPTY_BITBOARD {
            table_entry = eligible_moves;
        } else if table_entry != eligible_moves {
            return Err(LookupTableCreationError);
        }
    }
    Ok(lookup_table)
}

fn get_magic_index(magic_entry: &MagicEntry, bitset: BitBoard) -> usize {
    (magic_entry.number.wrapping_mul(bitset) >> magic_entry.shift) as usize
}




