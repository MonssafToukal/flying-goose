pub(crate) use int_enum::IntEnum;

#[repr(u8)]
#[derive(Debug, PartialEq, IntEnum)]
pub enum Pieces {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}
