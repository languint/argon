#[cfg(test)]
mod bitboard_tests {
    use argon_core::bitboard::{
        BB_FILE_A, BB_FILE_B, BB_FILE_C, BB_FILE_D, BB_FILE_E, BB_FILE_F, BB_FILE_G, BB_FILE_H,
        Bitboard,
    };

    #[test]
    fn empty_and_full() {
        assert_eq!(Bitboard::empty().bits(), 0);
        assert_eq!(Bitboard::full().bits(), u64::MAX);
    }

    #[test]
    fn set_and_get_bits() {
        let mut bb = Bitboard::empty();

        bb = bb.set(0).set(63);

        assert!(bb.get(0));
        assert!(bb.get(63));
        assert!(!bb.get(1));
        assert_eq!(bb.popcount(), 2);
    }

    #[test]
    fn clear_bits() {
        let mut bb = Bitboard::full();
        bb = bb.clear(0).clear(63);

        assert!(!bb.get(0));
        assert!(!bb.get(63));
        assert_eq!(bb.popcount(), 62);
    }

    #[test]
    fn bitwise_ops() {
        let a = Bitboard::new(0b1010);
        let b = Bitboard::new(0b1100);

        assert_eq!((a & b).bits(), 0b1000);
        assert_eq!((a | b).bits(), 0b1110);
        assert_eq!((a ^ b).bits(), 0b0110);
        assert_eq!((!a).bits(), !0b1010);
    }

    #[test]
    fn file_masks_are_correct() {
        assert_eq!(BB_FILE_A, 0x0101010101010101);
        assert_eq!(BB_FILE_H, 0x8080808080808080);

        for mask in [
            BB_FILE_A, BB_FILE_B, BB_FILE_C, BB_FILE_D, BB_FILE_E, BB_FILE_F, BB_FILE_G, BB_FILE_H,
        ] {
            assert_eq!(mask.count_ones(), 8);
        }
    }

    #[test]
    fn rank_masks_are_correct() {
        const BB_RANK_1: u64 = 0x00000000000000FF;
        const BB_RANK_8: u64 = 0xFF00000000000000;

        assert_eq!(BB_RANK_1.count_ones(), 8);
        assert_eq!(BB_RANK_8.count_ones(), 8);

        assert_eq!(BB_RANK_1 & BB_RANK_8, 0);
    }
}
