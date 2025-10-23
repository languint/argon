use crate::{castling::CastlingSide, square::Square};

/// Move flags, like castling, captures, en-passant, etc.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveFlags {
    Capture,
    EnPassant(Square),
    Castling(CastlingSide),
}

/// A chess move type
///
/// Holds a source [`Square`] and a destination [`Square`]
///
/// Along with [`MoveFlags`] like en-passant or castling
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Move(Square, Square, MoveFlags);

impl Move {
    /// Get the source [`Square`]
    #[must_use]
    #[inline]
    pub fn get_source(&self) -> Square {
        self.0
    }

    /// Get the destination [`Square`]
    #[must_use]
    #[inline]
    pub fn get_destination(&self) -> Square {
        self.1
    }

    /// Get the [`MoveFlags`]
    #[must_use]
    #[inline]
    pub fn get_flags(&self) -> MoveFlags {
        self.2
    }
}
