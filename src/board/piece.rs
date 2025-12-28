pub(crate) use int_enum::IntEnum;

pub const NUM_PIECE_TYPES: u64 = 6;

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, IntEnum)]
pub enum Piece {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}
