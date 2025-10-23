#[cfg(test)]
mod board_tests {
    use argon_core::{
        bitboard::{BB_RANK_2, BB_RANK_7, BB_RANK_8, Bitboard},
        board::{Board, STARTING_FEN},
        square::Square,
    };

    #[test]
    fn starting_fen() {
        let board = Board::try_from(STARTING_FEN).expect("Expected board construction to succeed");

        assert_eq!(board.bitboards[0], Bitboard(BB_RANK_2 | BB_RANK_7));

        assert_eq!(
            board.bitboards[1],
            Bitboard::squares_mask(vec![Square::B1, Square::B8, Square::G1, Square::G8])
        );

        assert_eq!(
            board.bitboards[2],
            Bitboard::squares_mask(vec![Square::C1, Square::C8, Square::F1, Square::F8])
        );

        assert_eq!(
            board.bitboards[3],
            Bitboard::squares_mask(vec![Square::A1, Square::A8, Square::H1, Square::H8])
        );

        assert_eq!(
            board.bitboards[4],
            Bitboard::squares_mask(vec![Square::D1, Square::D8])
        );

        assert_eq!(
            board.bitboards[5],
            Bitboard::squares_mask(vec![Square::E1, Square::E8])
        );

        assert_eq!(board.bitboards[6], Bitboard(BB_RANK_7 | BB_RANK_8));
    }
}
