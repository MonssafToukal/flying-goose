pub mod sliders;
pub mod nonsliders;
use std::fmt::Display;

use sliders::{defs::{BISHOP_SLIDER, ROOK_SLIDER, get_all_blockers_subsets}, magic_entries::{BISHOP_MAGICS, ROOK_MAGICS}, magics::{MAX_BISHOP_TABLE_SIZE, MAX_ROOK_TABLE_SIZE}};

use crate::{board::types::{EMPTY_BITBOARD, SquareCoord}, types::{BitBoard, NumOf}};


#[derive(Debug)]
pub enum MovementDataInitError {
    RookAttackCollisionError,
    BishopAttackCollisionError,
}

impl Display for MovementDataInitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            MovementDataInitError::RookAttackCollisionError => "Collision detected while initializing rook attack table",
            MovementDataInitError::BishopAttackCollisionError => "Collision detected while initializing bishop attack table",
        };
        write!(f, "{err}")
    }
}

pub struct MovementData {
    pub king_attacks: [BitBoard; NumOf::SQUARES],
    pub knight_attacks: [BitBoard; NumOf::SQUARES],
    pub rook_attacks: Vec<BitBoard>,
    pub bishop_attacks: Vec<BitBoard>,
}

impl MovementData {
    pub fn new() -> Self {
        Self {
            king_attacks: [EMPTY_BITBOARD; NumOf::SQUARES],
            knight_attacks: [EMPTY_BITBOARD; NumOf::SQUARES],
            rook_attacks: vec![EMPTY_BITBOARD; MAX_ROOK_TABLE_SIZE],
            bishop_attacks: vec![EMPTY_BITBOARD; MAX_BISHOP_TABLE_SIZE],
        }
    }

    pub fn init(&mut self) -> Result<(), MovementDataInitError> {
        self.init_king_attacks();
        self.init_knight_attacks();
        self.init_rook_attacks()?;
        self.init_bishop_attacks()?;
        Ok(())
    }

}


impl MovementData {
    fn init_king_attacks(&self) -> () {
        todo!()
    }

    fn init_knight_attacks(&mut self) -> () {
        todo!()
    }
    fn init_rook_attacks(&mut self) -> Result<(), MovementDataInitError> {
        for (square_idx, &magic_entry) in ROOK_MAGICS.iter().enumerate() {
            let square_coord = SquareCoord::try_from(square_idx as u8).unwrap();
            let blocker_mask = magic_entry.blocker_mask;
            for blocker_subset in get_all_blockers_subsets(blocker_mask) {
                let eligible_rook_moves = ROOK_SLIDER.get_moves(square_coord, blocker_subset);
                let magic_index = magic_entry.get_magic_index(blocker_subset);
                let rook_attack_slot = &mut self.rook_attacks[magic_index + magic_entry.offset as usize];
                if *rook_attack_slot != EMPTY_BITBOARD && *rook_attack_slot != eligible_rook_moves {
                    return Err(MovementDataInitError::RookAttackCollisionError);
                }
                *rook_attack_slot = eligible_rook_moves;
            }
        }
        Ok(())
    }

    fn init_bishop_attacks(&mut self) -> Result<(), MovementDataInitError> {
        for (square_idx, &magic_entry) in BISHOP_MAGICS.iter().enumerate() {
            let square_coord = SquareCoord::try_from(square_idx as u8).unwrap();
            let blocker_mask = magic_entry.blocker_mask;
            for blocker_subset in get_all_blockers_subsets(blocker_mask) {
                let eligible_bishop_moves = BISHOP_SLIDER.get_moves(square_coord, blocker_subset);
                let magic_index = magic_entry.get_magic_index(blocker_subset);
                let bishop_attack_slot = &mut self.bishop_attacks[magic_index + magic_entry.offset as usize];
                if *bishop_attack_slot != EMPTY_BITBOARD && *bishop_attack_slot != eligible_bishop_moves {
                    return Err(MovementDataInitError::BishopAttackCollisionError);
                }
                *bishop_attack_slot = eligible_bishop_moves;
            }
        }
        Ok(())

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sliders::{
        defs::{BISHOP_SLIDER, ROOK_SLIDER, get_all_blockers_subsets},
        magic_entries::{BISHOP_MAGICS, ROOK_MAGICS},
    };

    #[test]
    fn rook_init_has_no_collisions() {
        let mut md = MovementData::new();
        assert!(md.init_rook_attacks().is_ok());
    }

    #[test]
    fn bishop_init_has_no_collisions() {
        let mut md = MovementData::new();
        assert!(md.init_bishop_attacks().is_ok());
    }

    #[test]
    fn rook_lookup_returns_correct_moves() {
        let mut md = MovementData::new();
        md.init_rook_attacks().unwrap();
        for (sq_idx, &magic_entry) in ROOK_MAGICS.iter().enumerate() {
            let sq = SquareCoord::try_from(sq_idx as u8).unwrap();
            for blocker_subset in get_all_blockers_subsets(magic_entry.blocker_mask) {
                let expected = ROOK_SLIDER.get_moves(sq, blocker_subset);
                let idx = magic_entry.get_magic_index(blocker_subset);
                assert_eq!(
                    md.rook_attacks[idx + magic_entry.offset as usize],
                    expected,
                    "rook sq {sq_idx} blockers {blocker_subset:#018x}"
                );
            }
        }
    }

    #[test]
    fn bishop_lookup_returns_correct_moves() {
        let mut md = MovementData::new();
        md.init_bishop_attacks().unwrap();
        for (sq_idx, &magic_entry) in BISHOP_MAGICS.iter().enumerate() {
            let sq = SquareCoord::try_from(sq_idx as u8).unwrap();
            for blocker_subset in get_all_blockers_subsets(magic_entry.blocker_mask) {
                let expected = BISHOP_SLIDER.get_moves(sq, blocker_subset);
                let idx = magic_entry.get_magic_index(blocker_subset);
                assert_eq!(
                    md.bishop_attacks[idx + magic_entry.offset as usize],
                    expected,
                    "bishop sq {sq_idx} blockers {blocker_subset:#018x}"
                );
            }
        }
    }
}
