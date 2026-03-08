use crate::board::types::{Direction, EMPTY_BITBOARD, MAX_DIRECTIONS, SquareCoord};
use crate::movement::magic::MagicEntry;
use crate::types::{BitBoard, NumOf, SQUARE_MASKS};

pub struct Slider {
    pub directions: [Direction; MAX_DIRECTIONS],
}

pub const ROOK_SLIDER: Slider = Slider {
    directions: [(-1, 0), (1, 0), (0, -1), (0, 1)],
};

pub const BISHOP_SLIDER: Slider = Slider {
    directions: [(-1, -1), (-1, 1), (1, -1), (1, 1)],
};

impl Slider {
    pub fn get_moves(&self, square: SquareCoord, blocker_mask: BitBoard) -> BitBoard {
        let mut move_bitboard: BitBoard = EMPTY_BITBOARD;
        self.directions.iter().for_each(|direction| {
            let mut current_square = square;
            while let Ok(next_square) = current_square.next(*direction) {
                move_bitboard |= SQUARE_MASKS[next_square.to_usize()];
                if SQUARE_MASKS[next_square.to_usize()] & blocker_mask != 0 {
                    break;
                }
                current_square = next_square;
            }
        });
        move_bitboard
    }
    fn get_blocker_mask(&self, square: SquareCoord) -> BitBoard {
        let mut blockers_mask = EMPTY_BITBOARD;
        self.directions.iter().for_each(|direction| {
            let mut current_square = square;
            while let Ok(next_square) = current_square.next(*direction) {
                if let Ok(next_next_square) = next_square.next(*direction) {
                    blockers_mask |= SQUARE_MASKS[next_square.to_usize()];
                }
                current_square = next_square;
            }
        });
        blockers_mask
    }

    pub fn get_all_blockers(&self) -> [BitBoard; NumOf::SQUARES] {
        let mut slider_blockers_masks: [BitBoard; NumOf::SQUARES] =
            [EMPTY_BITBOARD; NumOf::SQUARES];
        for (square_idx, blocker_mask) in slider_blockers_masks.iter_mut().enumerate() {
            let current_square = SquareCoord::try_from(square_idx as u8).unwrap();
            *blocker_mask = self.get_blocker_mask(current_square);
        }
        slider_blockers_masks
    }

    pub fn find_magic_number(&self, square: SquareCoord) -> (MagicEntry, Vec<BitBoard>) {
        // 1. For each blocker configuration, find the eligible rook moves bitboard associated to it.
        let blocker_mask = self.get_blocker_mask(square);
        loop {
            let magic = MagicEntry::new(blocker_mask);
            if let Ok(lookup_table) = build_lookup_table(self, &magic, square) {
                return (magic, lookup_table);
            }
        }
        todo!()
    }
}

fn build_lookup_table(
    slider: &Slider,
    magic_entry: &MagicEntry,
    square: SquareCoord,
) -> Result<Vec<BitBoard>, ()> {
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
