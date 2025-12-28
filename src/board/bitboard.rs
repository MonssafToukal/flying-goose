pub type BitBoard = u64;

pub const NUM_SQUARES: u64 = 64;

pub fn print_bb(bitboard: BitBoard) -> () {
    const LAST_SQUARE_BIT: u64 = 63;
    // rank 0 is the h-rank
    for rank in 0..8 {
        for file in (0..8).rev() {
            let mask: u64 = 1u64 << (LAST_SQUARE_BIT - (rank * 8) - file);
            if mask & bitboard != 0 {
                print!("1");
            } else {
                print!("0");
            }
        }
        println!();
    }
}
