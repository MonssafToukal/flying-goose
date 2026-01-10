use std::fmt::Display;

use super::{
    defs::Board,
    types::{CastlingRight, NumOf, Pieces, Sides},
};

const FEN_START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const FEN_NR_PARTS: usize = 6;
const SHORT_FEN_NR_PARTS: usize = 4;
const SLASH: char = '/';
const SPACE: char = ' ';
const DASH: char = '-';
const EM_DASH: char = '—';

#[derive(Debug,PartialEq)]
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
    Ok(fen_parts)
}

pub fn fen_parse_pieces(board: &mut Board, part: &str) -> Result<(), FenError> {
    let fen_files: Vec<String> = part.split(SLASH).map(String::from).collect();
    if fen_files.len() != NumOf::RANKS {
        return Err(FenError::PiecePart);
    }
    let mut white_bb = board.bb_pieces[Sides::WHITE];
    let mut black_bb = board.bb_pieces[Sides::BLACK];

    for (i, fen_file) in fen_files.iter().enumerate() {
        let rank = NumOf::RANKS - i - 1;
        let mut file = 0;
        fen_file.chars().try_for_each(|c| {
            let square_idx = rank * 8 + file;
            let mut is_piece_match = true;
            match c {
                'k' => black_bb[Pieces::KING] |= 1u64 << square_idx,
                'q' => black_bb[Pieces::QUEEN] |= 1u64 << square_idx,
                'r' => black_bb[Pieces::ROOK] |= 1u64 << square_idx,
                'b' => black_bb[Pieces::BISHOP] |= 1u64 << square_idx,
                'n' => black_bb[Pieces::KNIGHT] |= 1u64 << square_idx,
                'p' => black_bb[Pieces::PAWN] |= 1u64 << square_idx,
                'K' => white_bb[Pieces::KING] |= 1u64 << square_idx,
                'Q' => white_bb[Pieces::QUEEN] |= 1u64 << square_idx,
                'R' => white_bb[Pieces::ROOK] |= 1u64 << square_idx,
                'B' => white_bb[Pieces::BISHOP] |= 1u64 << square_idx,
                'N' => white_bb[Pieces::KNIGHT] |= 1u64 << square_idx,
                'P' => white_bb[Pieces::PAWN] |= 1u64 << square_idx,
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

pub fn fen_parse_colour(board: &mut Board, part: &str) -> Result<(), FenError> {
  match part {
      "w" => board.game_state.active_color = Sides::WHITE as u8,
      "b" => board.game_state.active_color = Sides::BLACK as u8,
      _ => return Err(FenError::SidePart),
  }
    Ok(())
}

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

pub fn fen_parse_enpassant(board: &mut Board, part: &str) -> Result<(), FenError> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::board::fen::FEN_START_POSITION;

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
        let actual_start_position = split_fen_string(None).unwrap();
        assert_eq!(actual_start_position.len(), expected_start_position.len());

        for (expected_part, actual_part) in expected_start_position
            .iter()
            .zip(actual_start_position.iter())
        {
            assert_eq!(actual_part, expected_part);
        }
        // Test2: Incorrect length fen string
        let invalid_fen_string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR wKQkq - 0 1";
        let fen_split_res = split_fen_string(Some(invalid_fen_string));
        let error = fen_split_res.unwrap_err();
        assert_eq!(error, FenError::IncorrectLength);
    }
}
