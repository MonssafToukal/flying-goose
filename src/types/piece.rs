use super::errors::InvalidEnumConversionError;

pub enum Pieces {
    King = 0,
    Queen = 1,
    Rook = 2,
    Bishop = 3,
    Knight = 4,
    Pawn = 5,
    None = 6,
}

impl TryFrom<u8> for Pieces {
    type Error = InvalidEnumConversionError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::King),
            1 => Ok(Self::Queen),
            2 => Ok(Self::Rook),
            3 => Ok(Self::Bishop),
            4 => Ok(Self::Knight),
            5 => Ok(Self::Pawn),
            6 => Ok(Self::None),
            _ => Err(InvalidEnumConversionError),
        }
    }
}
