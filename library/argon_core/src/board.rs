use crate::{bitboard::Bitboard, color::Color, pieces::Pieces};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Board {
    pub bitboards: [Bitboard; 7],
}

pub const STARTING_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

impl Board {
    #[must_use]
    #[inline]
    pub fn get_piece_bitboard(piece: Pieces) -> usize {
        piece as usize
    }
}

impl TryFrom<&str> for Board {
    type Error = String;
    /// Try to parse a FEN `&str`
    /// 
    /// # Errors
    /// Returns `Err` if the FEN is invalid 
    fn try_from(fen: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = fen.split_whitespace().collect();
        if parts.is_empty() {
            return Err("Cannot build a Board from an empty FEN string!".to_string());
        }

        let mut board = Self::default();
        let board_part: Vec<&str> = parts[0].split('/').collect();
        if board_part.len() != 8 {
            return Err("FEN must have 8 ranks".to_string());
        }

        for (rank_idx, rank_str) in board_part.iter().enumerate() {
            let mut file_idx = 0;

            for c in rank_str.chars() {
                if c.is_ascii_digit() {
                    file_idx += unsafe { c.to_digit(10).unwrap_unchecked() as usize };
                    continue;
                }

                let piece = Pieces::try_from(&c).map_err(|_| format!("Invalid FEN piece: {c}!"))?;

                let sq = rank_idx * 8 + file_idx;

                let piece_bb = &mut board.bitboards[Board::get_piece_bitboard(piece)];
                *piece_bb = piece_bb.set(u8::try_from(sq).or(Err(format!("Failed to parse {sq} as a u8!")))?);

                let color = Color::from(&c);
                if color == Color::White {
                    board.bitboards[6] = board.bitboards[6].set(u8::try_from(sq).or(Err(format!("Failed to parse {sq} as a u8!")))?);
                }

                file_idx += 1;
            }
        }

        Ok(board)
    }
}
