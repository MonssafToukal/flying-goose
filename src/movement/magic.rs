use std::fmt::Display;

use crate::{
    board::types::{EMPTY_BITBOARD, SquareCoord},
    movement::sliders::{Slider, get_all_blockers_subsets},
    types::{BitBoard, NumOf},
};
use rand::{Rng, SeedableRng, seq::IndexedRandom};
use rand_pcg::Pcg64;

use super::sliders::{BISHOP_SLIDER, ROOK_SLIDER};


const RANDOM_SEED : u64 = 42069;
const ROOK_TABLE_SIZE: usize = 102400;
const BISHOP_TABLE_SIZE: usize = 5248;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagicEntry {
    number: u64,
    blocker_mask: BitBoard,
    offset: u32,
    index_bits: u8,
    shift: u8,
}

impl Display for MagicEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Magic number: {} offset: {} index_bits: {} shift: {}",
               self.number, self.offset, self.index_bits, self.shift
        )
    }
}

impl MagicEntry {
    pub fn new(rng: &mut Pcg64, blocker_mask: BitBoard) -> Self {
        let mut number_of_bits_set = 0;
        let mut temp_blocker = blocker_mask;
        while temp_blocker != 0 {
            number_of_bits_set += 1;
            temp_blocker &= temp_blocker - 1;
        }
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

pub fn get_rook_magics() -> ([MagicEntry; NumOf::SQUARES], [BitBoard; ROOK_TABLE_SIZE]) {
    let mut rng = Pcg64::seed_from_u64(RANDOM_SEED);
    let mut lookup_table: [BitBoard;ROOK_TABLE_SIZE] = [EMPTY_BITBOARD; ROOK_TABLE_SIZE];
    let mut magic_entries: [Option<MagicEntry>; NumOf::SQUARES] = [None; NumOf::SQUARES];
    let mut current_offset = 0u64;
    for square_idx in 0..NumOf::SQUARES {
        let square = SquareCoord::try_from(square_idx as u8).unwrap();
        let (magic_entry, table) = find_magic(&mut rng, &ROOK_SLIDER, square);
        magic_entries[square_idx] = Some(magic_entry);
        for (i, table_entry) in table.iter().enumerate() {
            lookup_table[i + current_offset as usize] = *table_entry;
        }
        current_offset += magic_entry.offset as u64;
    }
    let magic_entries: [MagicEntry; NumOf::SQUARES] = magic_entries.into_iter().collect::<Option<Vec<MagicEntry>>>()
        .and_then(|v| v.try_into().ok()).unwrap();

    (magic_entries, lookup_table)
}

pub fn get_bishop_magics() -> ([MagicEntry; NumOf::SQUARES], [BitBoard; BISHOP_TABLE_SIZE]) {
    let mut rng = Pcg64::seed_from_u64(RANDOM_SEED);
    let mut lookup_table: [BitBoard;BISHOP_TABLE_SIZE] = [EMPTY_BITBOARD; BISHOP_TABLE_SIZE];
    let mut magic_entries: [Option<MagicEntry>; NumOf::SQUARES] = [None; NumOf::SQUARES];
    let mut current_offset = 0u64;
    for square_idx in 0..NumOf::SQUARES {
        let square = SquareCoord::try_from(square_idx as u8).unwrap();
        let (magic_entry, table) = find_magic(&mut rng, &BISHOP_SLIDER, square);
        magic_entries[square_idx] = Some(magic_entry);
        for (i, table_entry) in table.iter().enumerate() {
            lookup_table[i + current_offset as usize] = *table_entry;
        }
        current_offset += magic_entry.offset as u64;
    }
    let magic_entries: [MagicEntry; NumOf::SQUARES] = magic_entries.into_iter().collect::<Option<Vec<MagicEntry>>>()
        .and_then(|v| v.try_into().ok()).unwrap();

    (magic_entries, lookup_table)
}

fn find_magic(rng: &mut Pcg64,slider: &Slider, square: SquareCoord) -> (MagicEntry, Vec<BitBoard>) {
    let blocker_mask = slider.get_blocker_mask(square);
    loop {
        let mut magic = MagicEntry::new(rng, blocker_mask);
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

fn get_magic_index(magic_entry: &MagicEntry, occupancy: BitBoard) -> usize {
    (magic_entry.number.wrapping_mul(occupancy) >> magic_entry.shift) as usize
}




