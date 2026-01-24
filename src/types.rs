pub type BitBoard = u64;

pub const SQUARE_MASKS: [u64; NumOf::SQUARES] = generate_square_masks();

const fn generate_square_masks() -> [u64; NumOf::SQUARES] {
    let mut square_masks = [0u64; NumOf::SQUARES];
    let mut i = 0;
    while i < NumOf::SQUARES {
        square_masks[i] = 1u64 << i;
        i += 1;
    }
    square_masks
}
