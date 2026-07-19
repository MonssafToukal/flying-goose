
#[derive(Debug)]
pub struct KingDirections;
impl KingDirections {
    pub const NORTH: i8 = 8;
    pub const SOUTH: i8 = -8;
    pub const EAST: i8 = 1;
    pub const WEST: i8 = -1;
}


// The first cardinal direction mentioned is the one that takes 2 squares
// e.g SouthEast => 2 squares down and one sqaure right
#[derive(Debug)]
pub struct KnightDirections;
impl KnightDirections {
    pub const NORTH_EAST: i8 = 17;
    pub const NORTH_WEST: i8 = 15;
    pub const SOUTH_EAST: i8 = -15;
    pub const SOUTH_WEST: i8 = -17;
    pub const EAST_NORTH: i8 = 10;
    pub const EAST_SOUTH: i8 = -6;
    pub const WEST_NORTH: i8 = 6;
    pub const WEST_SOUTH: i8 = -10;
}







