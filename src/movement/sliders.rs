use crate::board::types::{Direction, EMPTY_BITBOARD, MAX_DIRECTIONS, SquareCoord};
use crate::types::{BitBoard, SQUARE_MASKS};

pub struct Slider {
    pub directions: [Direction; MAX_DIRECTIONS],
}

impl Slider {
    pub fn get_moves(&self, square: SquareCoord, blocker_mask: BitBoard) -> BitBoard {
        let mut move_bitboard: BitBoard = EMPTY_BITBOARD;
        for direction in self.directions {
            let mut current_square: SquareCoord = square;
            while let Ok(next_square) = current_square.next(direction) {
                move_bitboard |= SQUARE_MASKS[next_square.to_usize()];
                // check if blocker exists on the next square
                if SQUARE_MASKS[next_square.to_usize()] & blocker_mask != 0 {
                    break;
                }
                current_square = next_square;
            }
        }
        move_bitboard
    }
    fn get_blockers(&self, square: SquareCoord) -> BitBoard {
        let mut blockers_mask = EMPTY_BITBOARD;
        let mut current_square = square;
        self.directions.iter().for_each(|direction| {
            while let Ok(next_square) = square.next(*direction) {
                blockers_mask |= SQUARE_MASKS[next_square.to_usize()];
                current_square = next_square;
            }
        });
        blockers_mask
    }
}
