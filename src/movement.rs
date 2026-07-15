pub mod sliders;
use std::fmt::Display;

use sliders::{defs::{BISHOP_SLIDER, ROOK_SLIDER, get_all_blockers_subsets}, magic_entries::{BISHOP_MAGICS, ROOK_MAGICS}, magics::{MAX_BISHOP_TABLE_SIZE, MAX_ROOK_TABLE_SIZE}};

use crate::{board::types::{EMPTY_BITBOARD, SquareCoord}, types::BitBoard};


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
    pub rook_attacks: Vec<BitBoard>,
    pub bishop_attacks: Vec<BitBoard>,
}

impl MovementData {
    pub fn new() -> Self {
        Self {
            rook_attacks: vec![EMPTY_BITBOARD; MAX_ROOK_TABLE_SIZE],
            bishop_attacks: vec![EMPTY_BITBOARD; MAX_BISHOP_TABLE_SIZE],
        }
    }

    pub fn init(&mut self) -> Result<(), MovementDataInitError> {
        self.init_rook_attacks()?;
        self.init_bishop_attacks()?;
        Ok(())
    }
}


impl MovementData {
    fn init_rook_attacks(&mut self) -> Result<(), MovementDataInitError> {
        for (square_idx, &magic_entry) in ROOK_MAGICS.iter().enumerate() {
            let square_coord = SquareCoord::try_from(square_idx as u8).unwrap();
            let blocker_mask = magic_entry.blocker_mask;
            for blocker_subset in get_all_blockers_subsets(blocker_mask) {
                let eligible_rook_moves = ROOK_SLIDER.get_moves(square_coord, blocker_mask);
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
                let eligible_bishop_moves = BISHOP_SLIDER.get_moves(square_coord, blocker_mask);
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

// TODO: test the init functions
