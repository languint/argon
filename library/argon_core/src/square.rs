use std::fmt;

use crate::{file::File, rank::Rank};

/// A chess square using little endian mappings.
/// 
/// `0` -> `A1`
/// `63` -> `H8`
#[rustfmt::skip]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Square {
    A1 = 0,  B1 = 1,  C1 = 2,  D1 = 3,  E1 = 4,  F1 = 5,  G1 = 6,  H1 = 7,
    A2 = 8,  B2 = 9,  C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
}

impl TryFrom<u8> for Square {
    type Error = String;
    /// Attempt to create a [`Square`] from a `u8`
    ///
    /// # Errors
    /// Returns `Err` if the value is too big to be a square
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 63 {
            return Err(format!("Cannot parse `{value}` as a square"));
        }

        unsafe { Ok(std::mem::transmute::<u8, Square>(value)) }
    }
}

impl From<Square> for u8 {
    fn from(sq: Square) -> Self {
        sq as u8
    }
}

impl Square {
    /// Get the [`File`] associated with this [`Square`]
    #[inline]
    #[must_use]
    pub fn file(self) -> File {
        unsafe { File::from_u8((self as u8) & 7) }
    }

    /// Get the [`Rank`] associated with this [`Square`]
    #[inline]
    #[must_use]
    pub fn rank(self) -> Rank {
        Rank(self as u8 >> 3)
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let file = (b'a' + u8::from(self.file())) as char;
        let rank = (b'1' + self.rank().0) as char;
        write!(f, "{file}{rank}")
    }
}
