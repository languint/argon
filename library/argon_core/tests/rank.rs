#[cfg(test)]
mod rank_tests {
    use std::ops::RangeInclusive;
    use argon_core::rank::{ALL_RANKS, Rank};

    #[test]
    fn from_char() {
        let numbers: RangeInclusive<char> = '1'..='8';

        for rank_char in numbers {
            Rank::try_from(&rank_char)
                .unwrap_or_else(|_| panic!("Expected cast to succeed for '{rank_char}'"));
        }
    }

    #[test]
    fn as_char() {
        let numbers: Vec<char> = ('1'..='8').collect();

        for (index, rank) in ALL_RANKS.iter().enumerate() {
            let number_rank = Rank::try_from(&numbers[index])
                .expect("Expected cast to succeed");
            assert_eq!(&number_rank, rank, "Mismatch at index {index}");
        }
    }
}
