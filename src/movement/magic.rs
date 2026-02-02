use crate::types::{BitBoard, NumOf};
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
