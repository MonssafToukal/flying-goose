use crate::types::BitBoard;

#[derive(Debug,Copy, Clone, PartialEq)]
pub struct MagicEntry {
    number: u64,
    occupancy_mask: BitBoard,
    shift: u8,
}
