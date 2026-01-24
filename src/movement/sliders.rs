use crate::{board::types::{EMPTY_BITBOARD, Files, NumOf, Ranks, SQUARE_MASKS, SquareCoord}, types::BitBoard};



pub fn generate_rook_occupancy_masks() -> [BitBoard; NumOf::SQUARES] {
    let mut root_occuopancy_masks:[BitBoard; NumOf::SQUARES] = [EMPTY_BITBOARD; NumOf::SQUARES];
    let nb_files: u8 = u8::try_from(NumOf::FILES).unwrap();
    for rank in Ranks::R1 as u8 .. Ranks::R8 as u8 {
        for file in Files::A as u8 .. Files::H as u8 {
            let mut occupancy_mask: BitBoard = EMPTY_BITBOARD;
            let mut current_rank = rank;
            let mut current_file = file;
            // Going in the right direction until I reach the last file
            while current_file != Files::H as u8 {
                let current_square_index = current_rank * nb_files + current_file;

                current_file += 1;
            }
            // Going in the left direction until I reach the last file
            while current_file != Files::A as u8 {
                let current_square_index = current_rank * nb_files + current_file;
                occupancy_mask |= SQUARE_MASKS[current_square_index as usize];
                current_file -= 1;
            }
        } 
    } 
    todo!()
}
