use std::fmt::Display;

use super::{
    defs::Board,
    types::{NumOf, Sides},
};

const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const FEN_NR_PARTS: usize = 6;
const SHORT_FEN_NR_PARTS: usize = 4;
const LIST_OF_PIECES: &str = "kqrnbpKQRNBP";
const SLASH: char = '/';
const SPACE: char = ' ';
const DASH: char = '-';
const EM_DASH: char = '—';

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

pub fn split_fen_string(fen_str: Option<&str>) -> Result<Vec<String>, FenError> {
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
    return Ok(fen_parts);
}

pub fn fen_parse_pieces(board: &mut Board, part: &str) -> Result<Vec<String>, FenError> {
    let fen_files: Vec<String> = part.split(SLASH).map(String::from).collect();
    if fen_files.len() != NumOf::RANKS {
        return Err(FenError::PiecePart);
    }
    for fen_file in fen_files {
        let mut squares_found = 0;
        fen_file.chars()
                .for_each(|c| {
                    match s {
                        'k' => ,
                        'q' => ,
                        'r' => ,
                        'b' => ,
                        'n' => ,
                        'p' => ,
                        'K' => ,
                        'Q' => ,
                        'R' => ,
                        'B' => ,
                        'N' => ,
                        'P' => ,
                        _   => fen_parse_numbers(c),
                    }
        });
    }
}


fn fen_parse_numbers(c: char) -> Result<usize, FenError> {
    match c {
        '1'..='8' => {
            Ok(c.to_digit(10).unwrap() as usize)
        },
        _ => Err(FenError::PiecePart),
    }
}
