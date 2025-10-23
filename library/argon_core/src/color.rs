use std::ops::Not;

/// A chess [`Color`] or side
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    White,
    Black,
}

/// All [`Colors`][`Color`]
pub const ALL_COLORS: [Color; 2] = [Color::White, Color::Black];

impl Not for Color {
    type Output = Self;
    #[inline]
    fn not(self) -> Self::Output {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black,
        }
    }
}
