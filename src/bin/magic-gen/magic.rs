use std::fmt::Display;
use std::sync::OnceLock;

use flying_goose::board::types::{Direction, EMPTY_BITBOARD, FULL_BITBOARD, Files, Ranks, Square, SquareCoord};
use flying_goose::movement::sliders::{Slider, get_all_blockers_subsets};
use flying_goose::types::print_bb;
use flying_goose::types::{BitBoard, NumOf};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use flying_goose::movement::sliders::{BISHOP_SLIDER, ROOK_SLIDER};

const RANDOM_SEED: u64 = 1290381293;
const ROOK_TABLE_SIZE: usize = 102400;
const BISHOP_TABLE_SIZE: usize = 5248;

// Constants for squares
type CornerVec = Vec<Square>;
type EdgeVec = Vec<Square>;
type InteriorVec = Vec<Square>;
type CornerArray = [Square; NumOf::CORNER_SQUARES];
type EdgeArray = [Square; NumOf::EDGE_SQUARES];
type InteriorArray = [Square; NumOf::INTERIOR_SQUARES];

// const ALL_SQUARE_ARRAYS: (CornerArray, EdgeArray, InteriorArray) = init_square_lists();
// static CORNER_SQUARES: CornerArray = [0; NumOf::CORNER_SQUARES];
// static EDGE_SQUARES: EdgeArray = [0; NumOf::EDGE_SQUARES];
// static INTERIOR_SQUARES: InteriorArray = [0; NumOf::INTERIOR_SQUARES];
//

#[derive(Debug)]
struct InitSquareArrError;

fn init_square_lists() -> Result<(CornerArray, EdgeArray, InteriorArray), InitSquareArrError> {
    let mut corner_squares = Vec::with_capacity(NumOf::CORNER_SQUARES);
    let mut edge_squares = Vec::with_capacity(NumOf::EDGE_SQUARES);
    let mut interior_squares = Vec::with_capacity(NumOf::INTERIOR_SQUARES);
    let directions: [Direction; 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    for square_idx in 0..NumOf::SQUARES {
        let square: SquareCoord = SquareCoord::try_from(square_idx as u8).unwrap();
        let num_valid_directions = directions
            .iter()
            .filter(|&&x| square.next(x).is_ok())
            .count();
        // println!("square: {} => {num_valid_directions} valid directions", square_idx);
        match num_valid_directions {
            4 => interior_squares.push(square_idx),
            3 => edge_squares.push(square_idx),
            2 => corner_squares.push(square_idx),
            _ => return Err(InitSquareArrError),
        };
    }
    let corner_squares: CornerArray = corner_squares.try_into().map_err(|_| InitSquareArrError)?;
    let edge_squares: EdgeArray = edge_squares.try_into().map_err(|_| InitSquareArrError)?;
    let interior_squares: InteriorArray = interior_squares
        .try_into()
        .map_err(|_| InitSquareArrError)?;

    Ok((corner_squares, edge_squares, interior_squares))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MagicEntry {
    pub number: u64,
    pub blocker_mask: BitBoard,
    pub inverse_blocker_mask: BitBoard,
    pub offset: u32,
    pub index_bits: u8,
    pub shift: u8,
}

impl Display for MagicEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Magic number: {} offset: {} index_bits: {} shift: {}",
            self.number, self.offset, self.index_bits, self.shift
        )
    }
}

impl Default for MagicEntry {
    fn default() -> Self {
        MagicEntry { number: 0, blocker_mask: EMPTY_BITBOARD, inverse_blocker_mask: FULL_BITBOARD, offset: NumOf::SQUARES as u32, index_bits: 0, shift: NumOf::SQUARES as u8 }
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
        let mut magic_numbers = [0u64; 3];
        rng.fill(&mut magic_numbers);
        let magic_number: u64 = magic_numbers.into_iter().reduce(|acc, m| acc & m).unwrap();
        let num_squares: u8 = u8::try_from(NumOf::SQUARES).unwrap();
        Self {
            number: magic_number,
            blocker_mask: blocker_mask,
            inverse_blocker_mask: !blocker_mask,
            offset: 0,
            index_bits: number_of_bits_set,
            shift: num_squares - number_of_bits_set,
        }
    }
}

pub fn get_slider_magics(slider: &Slider) -> (Vec<MagicEntry>, Vec<BitBoard>) {
    let mut rng = Pcg64::seed_from_u64(RANDOM_SEED);
    let mut global_table = vec![EMPTY_BITBOARD; ROOK_TABLE_SIZE];

    let mut magic_entries: Vec<MagicEntry> = vec![MagicEntry::default(); NumOf::SQUARES];
    let (corner_squares, edge_squares, interior_squares) =
        init_square_lists().expect("could not get square arrays");
    // Ordering squares by the size of the lookup table in decreasing order
    let ordered_squares_by_bitset: Vec<Square> = [
        corner_squares.as_slice(),
        edge_squares.as_slice(),
        interior_squares.as_slice(),
    ]
    .concat();

    for square_idx in ordered_squares_by_bitset {
        let mut attempts: u64 = 0;
        'find_magic: loop {
            attempts += 1;
            if attempts % 100_000 == 0 {
                let filled = global_table.iter().filter(|&&x| x != EMPTY_BITBOARD).count();
                eprintln!(
                    "  ...still searching for square {square_idx}: {attempts} magic attempts so far, global_table {:.1}% full",
                    100.0 * filled as f64 / global_table.len() as f64
                );
            }
            let (mut magic_entry, table) = find_magic(&mut rng, slider, square_idx);
            let max_table_size = 1 << magic_entry.index_bits;
            let mut found_offset = false;
            'find_offset: for offset in 0..=global_table.len() - max_table_size as usize {
                for (i, &table_entry) in table.iter().enumerate() {
                    if table_entry == EMPTY_BITBOARD {
                        continue;
                    }
                    let global_table_entry = global_table[i + offset];
                    if global_table_entry != EMPTY_BITBOARD && global_table_entry != table_entry {
                        continue 'find_offset;
                    }
                }
                for (i, &table_entry) in table.iter().enumerate() {
                    if table_entry != EMPTY_BITBOARD {
                        global_table[i + offset] = table_entry;
                    }
                }
                found_offset = true;
                magic_entry.offset = offset as u32;
                magic_entries[square_idx] = magic_entry;
                break 'find_offset;
            }
            if found_offset {
                println!(
                    "square {square_idx:>2}: bits={} offset={} ({attempts} attempts)",
                    magic_entry.index_bits, magic_entry.offset
                );
                break 'find_magic;
            }
        }
    }
    // Truncate the global_table at the end:
    let last_nonzero_attack = global_table
        .iter()
        .rposition(|&x| x != 0)
        .expect("global table for slider is full of zeros which is nonsensical. Crashing");
    global_table.truncate(last_nonzero_attack + 1);

    (magic_entries, global_table)
}

fn find_magic(rng: &mut Pcg64, slider: &Slider, square: Square) -> (MagicEntry, Vec<BitBoard>) {
    let square_coord: SquareCoord = SquareCoord::try_from(square as u8).unwrap();
    let blocker_mask = slider.get_blocker_mask(square_coord);
    loop {
        let mut magic = MagicEntry::new(rng, blocker_mask);
        if let Ok(lookup_table) = get_lookup_table(&slider, &magic, square) {
            return (magic, lookup_table);
        }
    }
}

struct LookupTableCreationError;

fn get_lookup_table(
    slider: &Slider,
    magic_entry: &MagicEntry,
    square: Square,
) -> Result<Vec<BitBoard>, LookupTableCreationError> {
    let mut lookup_table = vec![EMPTY_BITBOARD; (1usize << magic_entry.index_bits)];
    let square = SquareCoord::try_from(square as u8).unwrap();
    for blocker_subset in get_all_blockers_subsets(magic_entry.blocker_mask) {
        let eligible_moves = slider.get_moves(square, blocker_subset);
        let index = get_magic_index(&magic_entry, blocker_subset);
        let table_entry = &mut lookup_table[index];
        if *table_entry == EMPTY_BITBOARD {
            *table_entry = eligible_moves;
        } else if *table_entry != eligible_moves {
            return Err(LookupTableCreationError);
        }
    }
    Ok(lookup_table)
}

pub fn get_magic_index(magic_entry: &MagicEntry, occupancy: BitBoard) -> usize {
    ((occupancy | magic_entry.inverse_blocker_mask).wrapping_mul(magic_entry.number) >> magic_entry.shift) as usize
}

pub fn print_magics(slider: &Slider, slider_name: &str) -> () {
    let slider_name = slider_name.to_uppercase();
    let (magics, _) = get_slider_magics(slider);
    // println!("pub const {slider_name}:[u64;NumOf::SQUARES] = [");
    if let Some((last_magic, rest)) = magics.split_last() {
        rest.into_iter().for_each(|m| println!("{},", m.number));
        println!("{}];", last_magic.number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flying_goose::movement::sliders::{BISHOP_SLIDER, ROOK_SLIDER, get_all_blockers_subsets};

    #[test]
    fn rook_magic_lookup_is_correct() {
        std::thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(|| {
                let (entries, table) = get_slider_magics(&ROOK_SLIDER);
                for (sq_idx, entry) in entries.iter().enumerate() {
                    let sq = SquareCoord::try_from(sq_idx as u8).unwrap();
                    for blockers in get_all_blockers_subsets(entry.blocker_mask) {
                        let expected = ROOK_SLIDER.get_moves(sq, blockers);
                        let idx = get_magic_index(entry, blockers);
                        assert_eq!(
                            table[entry.offset as usize + idx],
                            expected,
                            "rook sq {sq_idx} blockers {blockers:#018x}"
                        );
                    }
                }
            })
            .unwrap()
            .join()
            .unwrap();
    }

    #[test]
    fn bishop_magic_lookup_is_correct() {
        std::thread::Builder::new()
            .stack_size(16 * 1024 * 1024)
            .spawn(|| {
                let (entries, table) = get_slider_magics(&BISHOP_SLIDER);
                for (sq_idx, entry) in entries.iter().enumerate() {
                    let sq = SquareCoord::try_from(sq_idx as u8).unwrap();
                    for blockers in get_all_blockers_subsets(entry.blocker_mask) {
                        let expected = BISHOP_SLIDER.get_moves(sq, blockers);
                        let idx = get_magic_index(entry, blockers);
                        assert_eq!(
                            table[entry.offset as usize + idx],
                            expected,
                            "bishop sq {sq_idx} blockers {blockers:#018x}"
                        );
                    }
                }
            })
            .unwrap()
            .join()
            .unwrap();
    }
}
