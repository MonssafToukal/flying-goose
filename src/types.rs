pub type BitBoard = u64;
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
