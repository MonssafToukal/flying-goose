use super::types::{CastlingState, EnpassantSquareIdx, Files, NumOf, Piece, Pieces, Side, Sides, Square};
use rand::{Rng, SeedableRng};
use rand_pcg::Pcg64;

const NUM_PIECE_HASHES: usize = NumOf::PIECE_TYPES * Sides::BOTH;
pub type ZobristKey = u64;
type PieceHashes = [[ZobristKey; NumOf::SQUARES]; NUM_PIECE_HASHES];
type CastlingHashes = [ZobristKey; NumOf::CASTLING_STATES];
type EnpassantHashes = [ZobristKey; NumOf::ENPASSANT_FILES];

#[derive(Debug, Clone, Copy)]
pub struct Zobrist {
    // 12 * 64 hashes
    pieces_hash: PieceHashes,
    // 1 hash for the black side
    side_hashes: u64,
    // 16 hashes for all the castling combinations
    castling_hashes: CastlingHashes,
    // 8 hashes; 1 for each file of the board
    enpassant_hashes: EnpassantHashes,
}

impl Zobrist {
    const DEFAULT_RNG_SEED: u64 = 42069;

    pub fn new(seed: Option<u64>) -> Self {
        let seed = match seed {
            Some(s) => s,
            None => Self::DEFAULT_RNG_SEED,
        };
        let mut rng = Pcg64::seed_from_u64(seed);
        let mut pieces_hash: PieceHashes = [[0u64; NumOf::SQUARES]; NUM_PIECE_HASHES];
        pieces_hash.iter_mut().for_each(|piece| {
            piece.iter_mut().for_each(|square| {
                *square = rng.random();
            });
        });

        let side_hashes: u64 = rng.random();

        let mut castling_hashes: CastlingHashes = [0u64; NumOf::CASTLING_STATES];
        castling_hashes.iter_mut().for_each(|c| {
            *c = rng.random();
        });

        let mut enpassant_hashes: EnpassantHashes = [0u64; NumOf::ENPASSANT_FILES];
        enpassant_hashes.iter_mut().for_each(|c| {
            *c = rng.random();
        });

        Zobrist {
            pieces_hash: pieces_hash,
            side_hashes: side_hashes,
            castling_hashes: castling_hashes,
            enpassant_hashes: enpassant_hashes,
        }
    }
    pub fn piece(&self, side: Side, piece_type: Piece, square: Square) -> ZobristKey {
        debug_assert!(side < Sides::BOTH, "Invalid side: {:?}", side);
        debug_assert!(piece_type < Pieces::NONE, "Invalid piece: {:?}", piece_type);
        debug_assert!(square < NumOf::SQUARES, "Invalid square: {:?}", square);
        let piece_index = piece_type + (side * 6);

        return self.pieces_hash[piece_index][square];
    }
    pub fn castling(&self, castling_state: CastlingState) -> ZobristKey {
        debug_assert!(
            castling_state < NumOf::CASTLING_STATES as u8,
            "Invalid castling state index: {}",
            castling_state
        );
        self.castling_hashes[castling_state as usize]
    }
    pub fn enpassant(&self, file: Files) -> ZobristKey {
        self.enpassant_hashes[file as usize]
    }
    pub fn side(&self) -> ZobristKey {
        self.side_hashes
    }
}
