use std::{fmt, ops::{BitAnd, BitOr, BitXor, Not}};

/// A bitboard type, a wrapper around `u64`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(pub u64);

impl From<u64> for Bitboard {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

// impl Into<u64> for Bitboard {
//     #[inline]
//     fn into(self) -> u64 {
//         self.0
//     }
// }

impl Bitboard {
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline]
    pub const fn full() -> Self {
        Self(u64::MAX)
    }
}

impl Bitboard {
    /// Get the value of a square
    #[inline]
    pub const fn get(self, sq: u8) -> bool {
        (self.0 >> sq) & 1 != 0
    }

    /// Set the value of a square
    #[inline]
    pub const fn set(self, sq: u8) -> Self {
        Self(self.0 | (1u64 << sq))
    }

    /// Clear the value of a square
    #[inline]
    pub const fn clear(self, sq: u8) -> Self {
        Self(self.0 & !(1u64 << sq))
    }

    /// Get the number of `1`s
    #[inline]
    pub const fn popcount(self) -> u32 {
        self.0.count_ones()
    }

    /// Get the `u64` associated with this [`Bitboard`]
    #[inline]
    pub const fn bits(self) -> u64 {
        self.0
    }
}


impl Not for Bitboard {
    type Output = Self;

    #[inline]
    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

impl BitAnd for Bitboard {
    type Output = Self;

    #[inline]
    fn bitand(self, rhs: Bitboard) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for Bitboard {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Bitboard) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitXor for Bitboard {
    type Output = Self;

    #[inline]
    fn bitxor(self, rhs: Bitboard) -> Self::Output {
        Self(self.0 ^ rhs.0)
    }
}

impl Default for Bitboard {
    fn default() -> Self {
        Self(0)
    }
}

impl From<Bitboard> for u64 {
    #[inline]
    fn from(bb: Bitboard) -> Self {
        bb.0
    }
}

impl fmt::Display for Bitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for rank in (0..8).rev() {
            for file in 0..8 {
                let sq = rank * 8 + file;
                let bit = (self.0 >> sq) & 1;
                write!(f, "{} ", if bit == 1 { '1' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
