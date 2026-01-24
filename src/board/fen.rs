use crate::{
    board::{
        Board,
        types::{
            CastlingRight, FIFTY_MOVE_RULE, Files, MAX_GAME_MOVES, Pieces, Ranks, Sides,
            SquareCoord,
        },
    },
    types::{NumOf, SQUARE_MASKS},
};
use std::{fmt::Display, iter::chain};

type FenParseFunc = fn(board: &mut Board, part: &str) -> Result<(), FenError>;

const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const FEN_NR_PARTS: usize = 6;
const SHORT_FEN_NR_PARTS: usize = 4;
const SLASH: char = '/';
const SPACE: char = ' ';
const DASH: char = '-';
const EM_DASH: char = '—';
pub const FEN_PARSE_FUNCS: [FenParseFunc; FEN_NR_PARTS] = [
    fen_parse_pieces,
    fen_parse_colour,
    fen_parse_castling_rights,
    fen_parse_enpassant,
    fen_parse_half_move_clock,
    fen_parse_full_move_counter,
];

#[derive(Debug, PartialEq)]
pub enum FenError {
    IncorrectLength,
    PiecePart,
    SidePart,
    CastlingPart,
    EnpassantPart,
    HalfMovePart,
    FullMovePart,
}

impl Display for FenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let err = match self {
            FenError::IncorrectLength => "FEN String error: Must have 6 parts",
            FenError::PiecePart => "Error encountered during the parsing of the piece part",
            FenError::SidePart => "Error encountered during the parsing of the side part",
            FenError::CastlingPart => "Error encountered during the parsing of the castling part",
            FenError::EnpassantPart => {
                "Error encountered during the parsing of the en passant part"
            }
            FenError::HalfMovePart => "Error encountered during the parsing of the half move part",
            FenError::FullMovePart => "Error encountered during the parsing of the full move part",
        };
        write!(f, "{err}")
    }
}

pub fn fen_split_string(fen_str: Option<&str>) -> Result<Vec<String>, FenError> {
    let fen_str = match fen_str {
        Some(s) => s,
        None => FEN_START_POSITION,
    };
    let mut fen_parts: Vec<String> = fen_str
        .trim()
        .replace(EM_DASH, DASH.encode_utf8(&mut [0u8; 4]))
        .split(SPACE)
        .map(String::from)
        .collect();
    if fen_parts.len() == SHORT_FEN_NR_PARTS {
        fen_parts.append(&mut vec![String::from("0"), String::from("1")]);
    }
    if fen_parts.len() != FEN_NR_PARTS {
        return Err(FenError::IncorrectLength);
    }
    Ok(fen_parts)
}

// TODO: Write unit test for this
pub fn fen_parse_pieces(board: &mut Board, part: &str) -> Result<(), FenError> {
    let fen_files: Vec<String> = part.split(SLASH).map(String::from).collect();
    if fen_files.len() != NumOf::RANKS {
        return Err(FenError::PiecePart);
    }
    for (i, fen_file) in fen_files.iter().enumerate() {
        let rank = NumOf::RANKS - i - 1;
        let mut file = 0;
        fen_file.chars().try_for_each(|c| {
            let square_idx = rank * 8 + file;
            let mut is_piece_match = true;
            match c {
                'k' => {
                    board.bb_pieces[Sides::BLACK][Pieces::KING] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::BLACK] |= SQUARE_MASKS[square_idx];
                }
                'q' => {
                    board.bb_pieces[Sides::BLACK][Pieces::QUEEN] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::BLACK] |= SQUARE_MASKS[square_idx];
                }
                'r' => {
                    board.bb_pieces[Sides::BLACK][Pieces::ROOK] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::BLACK] |= SQUARE_MASKS[square_idx];
                }
                'b' => {
                    board.bb_pieces[Sides::BLACK][Pieces::BISHOP] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::BLACK] |= SQUARE_MASKS[square_idx];
                }
                'n' => {
                    board.bb_pieces[Sides::BLACK][Pieces::KNIGHT] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::BLACK] |= SQUARE_MASKS[square_idx];
                }
                'p' => {
                    board.bb_pieces[Sides::BLACK][Pieces::PAWN] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::BLACK] |= SQUARE_MASKS[square_idx];
                }
                'K' => {
                    board.bb_pieces[Sides::WHITE][Pieces::KING] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::WHITE] |= SQUARE_MASKS[square_idx];
                }
                'Q' => {
                    board.bb_pieces[Sides::WHITE][Pieces::QUEEN] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::WHITE] |= SQUARE_MASKS[square_idx];
                }
                'R' => {
                    board.bb_pieces[Sides::WHITE][Pieces::ROOK] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::WHITE] |= SQUARE_MASKS[square_idx];
                }
                'B' => {
                    board.bb_pieces[Sides::WHITE][Pieces::BISHOP] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::WHITE] |= SQUARE_MASKS[square_idx];
                }
                'N' => {
                    board.bb_pieces[Sides::WHITE][Pieces::KNIGHT] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::WHITE] |= SQUARE_MASKS[square_idx];
                }
                'P' => {
                    board.bb_pieces[Sides::WHITE][Pieces::PAWN] |= SQUARE_MASKS[square_idx];
                    board.bb_sides[Sides::WHITE] |= SQUARE_MASKS[square_idx];
                }
                '1'..='8' => {
                    is_piece_match = false;
                    if let Some(num) = c.to_digit(10) {
                        file += num as usize;
                    } else {
                        return Err(FenError::PiecePart);
                    }
                }
                _ => return Err(FenError::PiecePart),
            }
            if is_piece_match {
                file += 1;
            }
            Ok(())
        })?;
        if file != NumOf::FILES {
            return Err(FenError::PiecePart);
        }
    }
    Ok(())
}

// TODO: Write unit test for this
pub fn fen_parse_colour(board: &mut Board, part: &str) -> Result<(), FenError> {
    match part {
        "w" => board.game_state.active_color = Sides::WHITE as u8,
        "b" => board.game_state.active_color = Sides::BLACK as u8,
        _ => return Err(FenError::SidePart),
    }
    Ok(())
}

// TODO: Write unit test for this
pub fn fen_parse_castling_rights(board: &mut Board, part: &str) -> Result<(), FenError> {
    if !(1..=4).contains(&part.len()) {
        return Err(FenError::CastlingPart);
    }
    part.chars().try_for_each(|c| {
        match c {
            'k' => board.game_state.castling |= CastlingRight::BlackKingSide as u8,
            'q' => board.game_state.castling |= CastlingRight::BlackQueenSide as u8,
            'K' => board.game_state.castling |= CastlingRight::WhiteKingSide as u8,
            'Q' => board.game_state.castling |= CastlingRight::WhiteQueenSide as u8,
            '-' => (),
            _ => return Err(FenError::CastlingPart),
        }
        Ok(())
    })?;
    Ok(())
}

// TODO: Write unit test for this
pub fn fen_parse_enpassant(board: &mut Board, part: &str) -> Result<(), FenError> {
    if part.len() == 1 {
        if let Some(c) = part.chars().nth(0)
            && c == DASH
        {
            return Ok(());
        }
        if part.contains(DASH) {}
        return Err(FenError::EnpassantPart);
    }
    if part.len() == 2 {
        let mut pchar = part.chars();
        let file: Files = match pchar.nth(0).unwrap() {
            'a' => Files::A,
            'b' => Files::B,
            'c' => Files::C,
            'd' => Files::D,
            'e' => Files::E,
            'f' => Files::F,
            'g' => Files::G,
            'h' => Files::H,
            _ => return Err(FenError::EnpassantPart),
        };
        let rank: Ranks = match pchar.nth(1).unwrap() {
            '3' => Ranks::R3,
            '6' => Ranks::R6,
            _ => return Err(FenError::EnpassantPart),
        };
        board.game_state.enpassant = Some(SquareCoord {
            file: file,
            rank: rank,
        });
        return Ok(());
    }
    Err(FenError::EnpassantPart)
}

// TODO: Write unit test for this
pub fn fen_parse_half_move_clock(board: &mut Board, part: &str) -> Result<(), FenError> {
    if let Ok(x) = part.parse::<u8>()
        && x <= FIFTY_MOVE_RULE
    {
        board.game_state.half_move_clock = x;
        return Ok(());
    }
    Err(FenError::HalfMovePart)
}

// TODO: Write unit test for this
pub fn fen_parse_full_move_counter(board: &mut Board, part: &str) -> Result<(), FenError> {
    if let Ok(x) = part.parse::<u16>()
        && x as u64 <= MAX_GAME_MOVES
    {
        board.game_state.fullmove_counter = x;
        return Ok(());
    }
    Err(FenError::FullMovePart)
}

#[cfg(test)]
mod tests {
    use crate::board::{
        fen::FEN_START_POSITION,
        history::GameHistory,
        state::GameState,
        types::{Files, SquareCoord},
        zobrist::Zobrist,
    };
    use crate::types::SQUARE_MASKS;

    use super::*;

    #[test]
    fn test_split_fen_string() {
        // Test 1: default position
        let expected_start_position: Vec<String> = FEN_START_POSITION
            .trim()
            .replace(EM_DASH, DASH.encode_utf8(&mut [0; 4]))
            .split(SPACE)
            .map(String::from)
            .collect();
        let actual_start_position = fen_split_string(None).unwrap();
        assert_eq!(actual_start_position.len(), expected_start_position.len());

        for (expected_part, actual_part) in expected_start_position
            .iter()
            .zip(actual_start_position.iter())
        {
            assert_eq!(actual_part, expected_part);
        }
        // Test2: Incorrect length fen string
        let invalid_fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR wKQkq - 0 1";
        let fen_split_res = fen_split_string(Some(invalid_fen_string));
        let error = fen_split_res.unwrap_err();
        assert_eq!(error, FenError::IncorrectLength);
    }

    #[test]
    fn test_fen_parse_pieces() {
        let mut test_board = Board {
            bb_pieces: [[0; NumOf::PIECE_TYPES]; Sides::BOTH],
            bb_sides: [0; Sides::BOTH],
            piece_list: [Pieces::NONE; NumOf::SQUARES],
            game_state: GameState::new(),
            history: GameHistory::new(),
            zobrist_hashmap: Zobrist::new(None),
        };

        // test start position
        let parts = fen_split_string(Some(FEN_START_POSITION)).unwrap();
        let res = fen_parse_pieces(&mut test_board, parts[0].as_str());
        assert!(!res.is_err());
        // check if the board has the right values
        // 1. Check the bb_sides array
        const WHITE_START_MASK: u64 = (1u64 << NumOf::PIECES_PER_SIDE) - 1;
        assert_eq!(test_board.bb_sides[Sides::WHITE], WHITE_START_MASK);
        const BLACK_START_MASK: u64 = !((1u64 << (NumOf::SQUARES - NumOf::PIECES_PER_SIDE)) - 1);
        assert_eq!(test_board.bb_sides[Sides::BLACK], BLACK_START_MASK);
        let white_pieces = test_board.bb_pieces[Sides::WHITE];
        let black_pieces = test_board.bb_pieces[Sides::BLACK];
        for (piece_type, (wp, bp)) in white_pieces.iter().zip(black_pieces.iter()).enumerate() {
            match piece_type {
                Pieces::KING => {
                    let white_king_square = SquareCoord {
                        file: Files::E,
                        rank: Ranks::R1,
                    };
                    let black_king_square = SquareCoord {
                        file: Files::E,
                        rank: Ranks::R8,
                    };
                    let white_king_square_idx = white_king_square.to_usize();
                    let black_king_square_idx = black_king_square.to_usize();
                    assert_eq!(*wp, SQUARE_MASKS[white_king_square_idx]);
                    assert_eq!(*bp, SQUARE_MASKS[black_king_square_idx]);
                }

                Pieces::QUEEN => {
                    let white_queen_square = SquareCoord {
                        file: Files::D,
                        rank: Ranks::R1,
                    };
                    let black_queen_square = SquareCoord {
                        file: Files::D,
                        rank: Ranks::R8,
                    };
                    let white_queen_square_idx = white_queen_square.to_usize();
                    let black_queen_square_idx = black_queen_square.to_usize();
                    assert_eq!(*wp, SQUARE_MASKS[white_queen_square_idx]);
                    assert_eq!(*bp, SQUARE_MASKS[black_queen_square_idx]);
                }
                Pieces::ROOK => {
                    let white_rook_squares: [SquareCoord; 2] = [
                        SquareCoord {
                            file: Files::A,
                            rank: Ranks::R1,
                        },
                        SquareCoord {
                            file: Files::H,
                            rank: Ranks::R1,
                        },
                    ];
                    let white_rook_mask = SQUARE_MASKS[white_rook_squares[0].to_usize()]
                        | SQUARE_MASKS[white_rook_squares[1].to_usize()];
                    let black_rook_squares: [SquareCoord; 2] = [
                        SquareCoord {
                            file: Files::A,
                            rank: Ranks::R8,
                        },
                        SquareCoord {
                            file: Files::H,
                            rank: Ranks::R8,
                        },
                    ];
                    let black_rook_mask = SQUARE_MASKS[black_rook_squares[0].to_usize()]
                        | SQUARE_MASKS[black_rook_squares[1].to_usize()];
                    assert_eq!(*wp, white_rook_mask);
                    assert_eq!(*bp, black_rook_mask);
                }
                Pieces::BISHOP => {
                    let white_bishop_squares: [SquareCoord; 2] = [
                        SquareCoord {
                            file: Files::C,
                            rank: Ranks::R1,
                        },
                        SquareCoord {
                            file: Files::F,
                            rank: Ranks::R1,
                        },
                    ];
                    let white_bishop_mask = SQUARE_MASKS[white_bishop_squares[0].to_usize()]
                        | SQUARE_MASKS[white_bishop_squares[1].to_usize()];
                    let black_bishop_squares: [SquareCoord; 2] = [
                        SquareCoord {
                            file: Files::C,
                            rank: Ranks::R8,
                        },
                        SquareCoord {
                            file: Files::F,
                            rank: Ranks::R8,
                        },
                    ];
                    let black_bishop_mask = SQUARE_MASKS[black_bishop_squares[0].to_usize()]
                        | SQUARE_MASKS[black_bishop_squares[1].to_usize()];
                    assert_eq!(*wp, white_bishop_mask);
                    assert_eq!(*bp, black_bishop_mask);
                }
                Pieces::KNIGHT => {
                    let white_knight_squares: [SquareCoord; 2] = [
                        SquareCoord {
                            file: Files::B,
                            rank: Ranks::R1,
                        },
                        SquareCoord {
                            file: Files::G,
                            rank: Ranks::R1,
                        },
                    ];
                    let white_knight_mask = SQUARE_MASKS[white_knight_squares[0].to_usize()]
                        | SQUARE_MASKS[white_knight_squares[1].to_usize()];
                    let black_knight_squares: [SquareCoord; 2] = [
                        SquareCoord {
                            file: Files::B,
                            rank: Ranks::R8,
                        },
                        SquareCoord {
                            file: Files::G,
                            rank: Ranks::R8,
                        },
                    ];
                    let black_knight_mask = SQUARE_MASKS[black_knight_squares[0].to_usize()]
                        | SQUARE_MASKS[black_knight_squares[1].to_usize()];
                    assert_eq!(*wp, white_knight_mask);
                    assert_eq!(*bp, black_knight_mask);
                }
                Pieces::PAWN => {
                    let white_pawn_masks = ((1u64 << 16) - 1) & !((1u64 << 8) - 1);
                    let black_pawn_masks = (1u64 << 56) - 1 & !((1u64 << 48) - 1);
                    assert_eq!(*wp, white_pawn_masks);
                    assert_eq!(*bp, black_pawn_masks);
                }
                _ => {}
            }
        }
    }
}
