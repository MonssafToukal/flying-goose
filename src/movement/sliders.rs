use crate::board::types::{Direction, EMPTY_BITBOARD, MAX_DIRECTIONS, SquareCoord};
use crate::types::{BitBoard, NumOf, SQUARE_MASKS};

pub const ROOK_TABLE_SIZE: usize = 102400;
pub const BISHOP_TABLE_SIZE: usize = 5248;
pub const ROOK_MAGICS: [u64; NumOf::SQUARES] = [
    36029381413453824,
    18023332042506432,
    72075771415364864,
    4935949898890752000,
    144119621019107344,
    3602897319858079748,
    36030447393702144,
    1765412153445064832,
    4614078566471041072,
    3548977381404360720,
    10090455840109760512,
    5911043249318146,
    10696087839047760,
    2307531902023895056,
    577586656572479504,
    140738578876544,
    16142062698534797440,
    220676656690372608,
    54608345059853248,
    4611722302579605528,
    9511743700317775872,
    145241637805752448,
    2433082892861079848,
    2017614832094216772,
    144185649161994272,
    10522811171036332098,
    3458799700341162113,
    4503741362405448,
    2486721541090836736,
    9372274500705714304,
    1441154096964845832,
    11530345627489698819,
    9226820105864741264,
    5188217276918140928,
    72480974742495234,
    3481282653708095560,
    2314859006717330432,
    5242475841438372352,
    9079628005680,
    4543223418585153,
    3476779463161708548,
    18031993514106880,
    3533109093168381968,
    2630154959076753536,
    5206310702905622545,
    265716776094957696,
    290501971738361896,
    1188953067656183809,
    72341817797910784,
    13838506192468443200,
    4503964162818176,
    9367856665134631680,
    288323834774294784,
    4505798717767808,
    281526549874944,
    74872625326920192,
    144678483778347778,
    282050506662482,
    4620728403144771603,
    4904455264746424577,
    1167839747161718787,
    6926818268808349729,
    4611692617913141324,
    4035225407925026898,
];

pub const BISHOP_MAGICS: [u64; NumOf::SQUARES] = [
    1198604494821146752,
    6757600678969344,
    2260596989362208,
    9368635121713807360,
    1130437606901890,
    9157146360938497,
    4684026535385006080,
    4613375022940554304,
    2486136605333652496,
    40546707821953536,
    17733953544224,
    144972961907089410,
    583220562810110464,
    297801694263377920,
    5764856042745761952,
    319964360737544,
    4904420016218710272,
    9007207911866880,
    1125908499005956,
    1155173352210440196,
    2615465520247341056,
    281483835188224,
    587191988072417283,
    288793431390815040,
    657560867675853060,
    2832428258363394,
    9261670501959368736,
    2816949327759368,
    2534374303088640,
    9224499038692311552,
    3941242383896584,
    1158270629851579138,
    73185967930347520,
    1227811462157697280,
    4525632870680578,
    324294907852751360,
    290836235889541264,
    23081231584919688,
    4936512543897180184,
    4622386466260058496,
    73271463737694344,
    4758353242818088960,
    640074243085846528,
    721704177150984456,
    10168292283024388,
    18031991773462656,
    1154056426377708544,
    36314721585992324,
    1271108994105366,
    153690293702492288,
    564094697742464,
    19808934952988,
    4616277647739191304,
    17731856646144,
    13853362729764553796,
    185791154137139328,
    143245883742722,
    41095638724732932,
    1513209476948690950,
    72200539143801344,
    2282620767568392,
    4981546749702312064,
    576812630520693312,
    1171015076744802336,
];

pub struct Slider {
    pub directions: [Direction; MAX_DIRECTIONS],
}

pub const ROOK_SLIDER: Slider = Slider {
    directions: [(-1, 0), (1, 0), (0, -1), (0, 1)],
};

pub const BISHOP_SLIDER: Slider = Slider {
    directions: [(-1, -1), (-1, 1), (1, -1), (1, 1)],
};

impl Slider {
    pub fn get_moves(&self, square: SquareCoord, blocker_mask: BitBoard) -> BitBoard {
        let mut move_bitboard: BitBoard = EMPTY_BITBOARD;
        self.directions.iter().for_each(|direction| {
            let mut current_square = square;
            while let Ok(next_square) = current_square.next(*direction) {
                move_bitboard |= SQUARE_MASKS[next_square.to_usize()];
                if SQUARE_MASKS[next_square.to_usize()] & blocker_mask != 0 {
                    break;
                }
                current_square = next_square;
            }
        });
        move_bitboard
    }

    pub fn get_blocker_mask(&self, square: SquareCoord) -> BitBoard {
        let mut blockers_mask = EMPTY_BITBOARD;
        self.directions.iter().for_each(|direction| {
            let mut current_square = square;
            while let Ok(next_square) = current_square.next(*direction) {
                if let Ok(_next_next_square) = next_square.next(*direction) {
                    blockers_mask |= SQUARE_MASKS[next_square.to_usize()];
                }
                current_square = next_square;
            }
        });
        blockers_mask
    }

    pub fn get_all_blockers(&self) -> [BitBoard; NumOf::SQUARES] {
        let mut slider_blockers_masks: [BitBoard; NumOf::SQUARES] =
            [EMPTY_BITBOARD; NumOf::SQUARES];
        for (square_idx, blocker_mask) in slider_blockers_masks.iter_mut().enumerate() {
            let current_square = SquareCoord::try_from(square_idx as u8).unwrap();
            *blocker_mask = self.get_blocker_mask(current_square);
        }
        slider_blockers_masks
    }
}

pub fn get_all_blockers_subsets(blocker_mask: BitBoard) -> Vec<BitBoard> {
    // This is just to reduce the number of allocations
    let mut subsets: Vec<BitBoard> = Vec::with_capacity(NumOf::SQUARES);
    subsets.push(EMPTY_BITBOARD);
    while let Some(current_subset) = subsets.last()
        && *current_subset != blocker_mask
    {
        let next_subset: BitBoard = current_subset.wrapping_sub(blocker_mask) & blocker_mask;
        subsets.push(next_subset);
    }
    subsets
}
