use crate::file::File;

/// A castling side
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastlingSide {
    KingSide,
    QueenSide,
}

impl CastlingSide {
    /// Get the file that the king would move to with a castling move
    #[must_use]
    #[inline]
    pub fn get_king_target_file(&self) -> File {
        if self == &Self::KingSide {
            File::G
        } else {
            File::C
        }
    }

    /// Get the file that the rook would move to with a castling move
    #[must_use]
    #[inline]
    pub fn get_rook_target_file(&self) -> File {
        if self == &Self::KingSide {
            File::F
        } else {
            File::D
        }
    }
}

/// Represents the castling rights for both sides of the board
pub type CastlingRights = [[Option<CastlingSide>; 2]; 2];