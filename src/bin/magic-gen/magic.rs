use std::collections::HashSet;
use std::fmt::Display;
use std::sync::OnceLock;

use flying_goose::board::types::{
    Direction, EMPTY_BITBOARD, FULL_BITBOARD, Files, Ranks, Square, SquareCoord,
};
use flying_goose::movement::sliders::{Slider, get_all_blockers_subsets};
use flying_goose::types::print_bb;
use flying_goose::types::{BitBoard, NumOf};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

use flying_goose::movement::sliders::{BISHOP_SLIDER, ROOK_SLIDER};

pub const RANDOM_SEED: u64 = 1290381293;
const ROOK_TABLE_SIZE: usize = 102400;
const BISHOP_TABLE_SIZE: usize = 5248;

// Constants for squares
type CornerVec = Vec<Square>;
type EdgeVec = Vec<Square>;
type InteriorVec = Vec<Square>;
type CornerArray = [Square; NumOf::CORNER_SQUARES];
type EdgeArray = [Square; NumOf::EDGE_SQUARES];
type InteriorArray = [Square; NumOf::INTERIOR_SQUARES];

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
        MagicEntry {
            number: 0,
            blocker_mask: EMPTY_BITBOARD,
            inverse_blocker_mask: FULL_BITBOARD,
            offset: NumOf::SQUARES as u32,
            index_bits: 0,
            shift: NumOf::SQUARES as u8,
        }
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

pub fn get_slider_magics(slider: &Slider, seed: u64, verbose: bool) -> (Vec<MagicEntry>, Vec<BitBoard>) {
    let mut rng = Pcg64::seed_from_u64(seed);
    // Sized to the classical no-sharing total: every square placed back-to-back with
    // zero overlap always fits, so this is a safe hard upper bound no matter how the
    // search below goes.
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
        let (mut magic_entry, table, attempts) = find_magic(&mut rng, slider, square_idx, verbose);
        let fill = table.iter().filter(|&&bb| bb != EMPTY_BITBOARD).count();

        let offset = (0..=global_table.len() - table.len())
            .find(|&candidate| {
                table.iter().enumerate().all(|(i, &table_entry)| {
                    table_entry == EMPTY_BITBOARD || {
                        let existing = global_table[i + candidate];
                        existing == EMPTY_BITBOARD || existing == table_entry
                    }
                })
            })
            .expect(
                "global_table is sized to the classical no-sharing total, so placing every square back-to-back with zero overlap is always a valid fallback",
            );

        for (i, &table_entry) in table.iter().enumerate() {
            if table_entry != EMPTY_BITBOARD {
                global_table[i + offset] = table_entry;
            }
        }
        magic_entry.offset = offset as u32;

        if verbose {
            println!(
                "square {square_idx:>2}: bits={} offset={} table_len={} fill={:.1}% ({attempts} attempts)",
                magic_entry.index_bits,
                offset,
                table.len(),
                100.0 * fill as f64 / table.len() as f64
            );
        }
        magic_entries[square_idx] = magic_entry;
    }

    let last_nonzero_attack = global_table
        .iter()
        .rposition(|&x| x != EMPTY_BITBOARD)
        .expect("global table for slider is full of zeros which is nonsensical. Crashing");
    global_table.truncate(last_nonzero_attack + 1);

    (magic_entries, global_table)
}

/// Draws candidate magics until one is collision-safe, then keeps searching for one that
/// maximizes constructive collisions instead of avoiding them: the fewer distinct slots
/// a square's table actually touches, the more empty slots are left for other squares to
/// overlap into in the shared global table. `distinct_values` (the number of genuinely
/// different attack bitboards this square can produce) is the theoretical floor on how
/// low the fill count can possibly go, so we stop early if we ever hit it exactly.
///
/// Two independent, finite bounds guarantee this always terminates: it gives up after
/// `MAX_ATTEMPTS_WITHOUT_IMPROVEMENT` consecutive attempts with no improvement, or after
/// `MAX_TOTAL_ATTEMPTS` attempts total, whichever comes first — so a square that keeps
/// finding rare, tiny improvements forever can't turn this into an unbounded search.
fn find_magic(rng: &mut Pcg64, slider: &Slider, square: Square, verbose: bool) -> (MagicEntry, Vec<BitBoard>, u32) {
    const MAX_ATTEMPTS_WITHOUT_IMPROVEMENT: u32 = 100_000;
    const MAX_TOTAL_ATTEMPTS: u32 = 5_000_000;
    const PROGRESS_INTERVAL: u32 = 50_000;

    let square_coord: SquareCoord = SquareCoord::try_from(square as u8).unwrap();
    let blocker_mask = slider.get_blocker_mask(square_coord);
    let distinct_values: usize = get_all_blockers_subsets(blocker_mask)
        .into_iter()
        .map(|subset| slider.get_moves(square_coord, subset))
        .collect::<HashSet<_>>()
        .len();

    let mut best: Option<(MagicEntry, Vec<BitBoard>, usize)> = None;
    let mut attempts_since_improvement: u32 = 0;
    let mut total_attempts: u32 = 0;

    loop {
        total_attempts += 1;
        let magic = MagicEntry::new(rng, blocker_mask);
        let Ok(lookup_table) = get_lookup_table(slider, &magic, square) else {
            continue;
        };

        let fill = lookup_table.iter().filter(|&&bb| bb != EMPTY_BITBOARD).count();
        let is_better = match &best {
            None => true,
            Some((_, _, best_fill)) => fill < *best_fill,
        };
        if is_better {
            if verbose {
                eprintln!(
                    "  square {square}: new best fill {fill} (target {distinct_values}) after {total_attempts} attempts"
                );
            }
            best = Some((magic, lookup_table, fill));
            attempts_since_improvement = 0;
            if fill == distinct_values {
                let (magic, lookup_table, _) = best.unwrap();
                return (magic, lookup_table, total_attempts);
            }
        } else {
            attempts_since_improvement += 1;
        }

        if attempts_since_improvement >= MAX_ATTEMPTS_WITHOUT_IMPROVEMENT
            || total_attempts >= MAX_TOTAL_ATTEMPTS
        {
            let (magic, lookup_table, _) = best.expect("set on the first valid magic above");
            return (magic, lookup_table, total_attempts);
        }

        if verbose && total_attempts % PROGRESS_INTERVAL == 0 {
            let best_fill = best.as_ref().map(|(_, _, fill)| *fill);
            eprintln!(
                "  square {square}: {total_attempts} attempts so far, best fill {best_fill:?}                  (target {distinct_values}), {attempts_since_improvement} since last improvement"
            );
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
    let last_slider_attack_idx = lookup_table
        .iter()
        .rposition(|&bb| bb != EMPTY_BITBOARD)
        .expect("local table has EMPTY_BITBOARD as entries");
    lookup_table.truncate(last_slider_attack_idx + 1);

    Ok(lookup_table)
}

pub fn get_magic_index(magic_entry: &MagicEntry, occupancy: BitBoard) -> usize {
    ((occupancy | magic_entry.inverse_blocker_mask).wrapping_mul(magic_entry.number)
        >> magic_entry.shift) as usize
}

pub fn print_magic_entries(entries: &[MagicEntry], slider_name: &str) {
    let slider_name = slider_name.to_uppercase();
    println!(
        "pub const {slider_name}_MAGICS: [MagicEntry; {}] = [",
        entries.len()
    );
    for entry in entries {
        println!(
            "    MagicEntry {{ number: {}, blocker_mask: {:#018x}, inverse_blocker_mask: {:#018x}, offset: {}, index_bits: {}, shift: {} }},",
            entry.number,
            entry.blocker_mask,
            entry.inverse_blocker_mask,
            entry.offset,
            entry.index_bits,
            entry.shift
        );
    }
    println!("];");
}

pub fn print_magics(slider: &Slider, slider_name: &str) -> () {
    let (magics, _) = get_slider_magics(slider, RANDOM_SEED, true);
    print_magic_entries(&magics, slider_name);
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
                let (entries, table) = get_slider_magics(&ROOK_SLIDER, RANDOM_SEED, true);
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
                let (entries, table) = get_slider_magics(&BISHOP_SLIDER, RANDOM_SEED, true);
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
