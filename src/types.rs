pub type BitBoard = u64;
pub const EMPTY_BITBOARD: BitBoard = 0;
pub const FULL_BITBOARD: BitBoard = BitBoard::MAX;

pub fn print_bb(bitboard: BitBoard) -> () {
    const LAST_SQUARE_BIT: u64 = 63;
    // rank 0 is the last rank from white side pov
    for rank in 0..8 {
        print!("{}  ", NumOf::RANKS - rank as usize);
        for file in (0..8).rev() {
            let mask: u64 = 1u64 << (LAST_SQUARE_BIT - (rank * 8) - file);
            if mask & bitboard != 0 {
                print!("1 ");
            } else {
                print!("0 ");
            }
        }
        println!();
    }
    println!();
    println!("   a b c d e f g h");
}

pub struct NumOf;
impl NumOf {
    pub const SQUARES: usize = 64;
    pub const CORNER_SQUARES: usize = 4;
    pub const EDGE_SQUARES: usize = 24;
    pub const INTERIOR_SQUARES: usize = 36;
    pub const PIECES_PER_SIDE: usize = 16;
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_STATES: usize = 16;
    pub const ENPASSANT_FILES: usize = 8;
    pub const RANKS: usize = 8;
    pub const FILES: usize = 8;
}

pub const SQUARE_MASKS: [BitBoard; NumOf::SQUARES] = generate_square_masks();

const fn generate_square_masks() -> [BitBoard; NumOf::SQUARES] {
    let mut square_masks = [0u64; NumOf::SQUARES];
    let mut i = 0;
    while i < NumOf::SQUARES {
        square_masks[i] = 1u64 << i;
        i += 1;
    }
    square_masks
}

pub const FILE_MASKS: [BitBoard; NumOf::FILES] = generate_file_masks();
pub const RANK_MASKS: [BitBoard; NumOf::RANKS] = generate_rank_masks();

const fn generate_file_masks() -> [BitBoard; NumOf::FILES] {
    let mut masks = [EMPTY_BITBOARD; NumOf::FILES];
    let mut current_file = 0;
    while current_file < NumOf::FILES {
        let mut current_rank = 0;
        while current_rank < NumOf::RANKS {
            masks[current_file] |= 1u64 << (current_rank * 8 + current_file);
            current_rank += 1;
        }
        current_file += 1;
    }
    masks
}

const fn generate_rank_masks() -> [BitBoard; NumOf::RANKS] {
    let mut masks = [EMPTY_BITBOARD; NumOf::RANKS];
    let mut current_rank = 0;
    while current_rank < NumOf::RANKS {
        let mut current_file = 0;
        while current_file < NumOf::FILES {
            masks[current_rank] |= 1u64 << (current_rank * 8 + current_file);
            current_file += 1;
        }
        current_rank += 1;
    }
    masks
}
