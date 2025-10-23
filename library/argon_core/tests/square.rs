#[cfg(test)]
mod square_tests {
    use argon_core::square::{ALL_SQUARES, Square};
    use std::ops::RangeInclusive;

    #[test]
    fn from_u8() {
        let numbers: RangeInclusive<u8> = 0..=63;
        for sq_u8 in numbers {
            Square::try_from(sq_u8)
                .unwrap_or_else(|_| panic!("Expected cast to succeed for '{sq_u8}'"));
        }
    }

    #[test]
    fn corners_eq() {
        assert_eq!(0u8, Square::A1.into());
        assert_eq!(63u8, Square::H8.into());
    }

    #[test]
    fn all_squares_complete() {
        assert_eq!(ALL_SQUARES.len(), 64);
        assert_eq!(ALL_SQUARES[0], Square::A1);
        assert_eq!(ALL_SQUARES[63], Square::H8);
    }
}
