pub type BitBoard = u64;
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

pub const MAX_GAME_MOVES: u64 = 2048;

pub type Piece = usize;
pub type Square = usize;
pub type Side = usize;
pub type CastlingState = u8;
pub type EnpassantState = u8;

pub struct Sides;
impl Sides {
    pub const WHITE: Side = 0;
    pub const BLACK: Side = 1;
    pub const BOTH: Side = 2;
}

pub struct NumOf;
impl NumOf {
    pub const SQUARES: usize = 64;
    pub const PIECE_TYPES: usize = 6;
    pub const CASTLING_STATES: usize = 16;
    pub const ENPASSANT_FILES: usize = 8;
}

#[repr(u8)]
pub enum CastlingRight {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8,
}
