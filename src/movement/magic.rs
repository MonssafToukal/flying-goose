use crate::{
    board::types::{EMPTY_BITBOARD, Files, Ranks, SquareCoord},
    movement::sliders::Slider,
    types::{BitBoard, NumOf, SQUARE_MASKS},
};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;


const RANDOM_SEED : u64 = 42069;

#[derive(Debug, Clone, PartialEq)]
pub struct MagicEntry {
    number: u64,
    blocker_mask: BitBoard,
    shift: u8,
}

impl MagicEntry {
    pub fn new(blocker_mask: BitBoard) -> Self {
        let mut number_of_bits_set = 0;
        let mut temp_blocker = blocker_mask;
        while blocker_mask != 0 {
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
            shift: num_squares - number_of_bits_set,
        }
    }
}

pub fn find_magic(slider: &Slider, square: SquareCoord) -> (MagicEntry, Vec<BitBoard>) {
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
    todo!()
}




