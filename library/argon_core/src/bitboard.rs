use std::{
    fmt,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
};

/// A bitboard type, a wrapper around `u64`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Bitboard(pub u64);

impl From<u64> for Bitboard {
    #[inline]
    fn from(value: u64) -> Self {
        Self(value)
    }
}

pub const BB_FILE_A: u64 = 0x0101_0101_0101_0101;
pub const BB_FILE_B: u64 = 0x0202_0202_0202_0202;
pub const BB_FILE_C: u64 = 0x0404_0404_0404_0404;
pub const BB_FILE_D: u64 = 0x0808_0808_0808_0808;
pub const BB_FILE_E: u64 = 0x1010_1010_1010_1010;
pub const BB_FILE_F: u64 = 0x2020_2020_2020_2020;
pub const BB_FILE_G: u64 = 0x4040_4040_4040_4040;
pub const BB_FILE_H: u64 = 0x8080_8080_8080_8080;

pub const BB_RANK_1: u64 = 0x0000_0000_0000_00FF;
pub const BB_RANK_2: u64 = 0x0000_0000_0000_FF00;
pub const BB_RANK_3: u64 = 0x0000_0000_00FF_0000;
pub const BB_RANK_4: u64 = 0x0000_0000_FF00_0000;
pub const BB_RANK_5: u64 = 0x0000_00FF_0000_0000;
pub const BB_RANK_6: u64 = 0x0000_FF00_0000_0000;
pub const BB_RANK_7: u64 = 0x00FF_0000_0000_0000;
pub const BB_RANK_8: u64 = 0xFF00_0000_0000_0000;

impl Bitboard {
    #[must_use]
    #[inline]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[must_use]
    #[inline]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[must_use]
    #[inline]
    pub const fn full() -> Self {
        Self(u64::MAX)
    }
}

impl Bitboard {
    /// Get the value of a square
    #[must_use]
    #[inline]
    pub const fn get(self, sq: u8) -> bool {
        (self.0 >> sq) & 1 != 0
    }

    /// Set the value of a square
    #[must_use]
    #[inline]
    pub const fn set(self, sq: u8) -> Self {
        Self(self.0 | (1u64 << sq))
    }

    /// Clear the value of a square
    #[must_use]
    #[inline]
    pub const fn clear(self, sq: u8) -> Self {
        Self(self.0 & !(1u64 << sq))
    }

    /// Get the number of `1`s
    #[must_use]
    #[inline]
    pub const fn popcount(self) -> u32 {
        self.0.count_ones()
    }

    /// Get the `u64` associated with this [`Bitboard`]
    #[must_use]
    #[inline]
    pub const fn bits(self) -> u64 {
        self.0
    }
}

impl Bitboard {
    #[must_use]
    #[inline]
    /// Get a [`Bitboard`] mask with a square
    pub const fn square_mask(sq: u8) -> Self {
        Self(1u64 << sq)
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

impl BitAndAssign for Bitboard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for Bitboard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for Bitboard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
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
