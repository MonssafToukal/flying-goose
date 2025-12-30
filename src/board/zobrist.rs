use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;
use super::{
    piece::Pieces,
    types::{NumOf, Piece, Side, Sides, Square},
};

const NUM_PIECE_HASHES: usize = NumOf::PIECE_TYPES * Sides::BOTH;
pub type ZobristKey = u64;
type PieceHashes = [[u64; NumOf::SQUARES]; NUM_PIECE_HASHES];
type CastlingHashes = [u64; NumOf::CASTLING_STATES];
type EnpassantHashes = [u64; NumOf::ENPASSANT_FILES];

pub struct Zobrist {
    pieces_hash: PieceHashes,
    side_hashes: u64,
    castling_hashes: CastlingHashes,
    enpassant_hashes: EnpassantHashes,
}

impl Zobrist {
    const DEFAULT_RNG_SEED: u64 = 42069;

    pub fn new(seed: u64) -> Self {
        let mut rng = Pcg64::seed_from_u64(seed);
        let mut pieces_hash: PieceHashes = [[0u64; NumOf::SQUARES]; NUM_PIECE_HASHES]; 

        let side_hashes: u64 = rng.random();

    }
    pub fn piece(&self, colour: Side, piece_type: Piece, square: Square) -> ZobristKey {
        debug_assert!(colour < Sides::BOTH, "Invalid side: {:?}", colour);
        debug_assert!(piece_type < Pieces::NONE, "Invalid piece: {:?}", piece_type);
        debug_assert!(square < NumOf::SQUARES, "Invalid square: {:?}", square);
        let piece_index = piece_type + (colour * 6);

        return self.pieces_hash[piece_index][square];
    }
}
