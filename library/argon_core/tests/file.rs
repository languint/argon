#[cfg(test)]
mod file_tests {
    use argon_core::file::File;
    use std::ops::RangeInclusive;

    #[test]
    fn from_char() {
        let alphabet = String::from_utf8((b'a'..=b'h').chain(b'A'..=b'H').collect())
            .expect("Expected string building to succeed");

        for ch in alphabet.chars() {
            File::try_from(&ch).unwrap_or_else(|_| panic!("Expected cast to succeed for `{ch}`"));
        }
    }

    #[test]
    fn from_u8() {
        let numbers: RangeInclusive<u8> = 0..=7;

        for file_value in numbers {
            File::try_from(file_value)
                .unwrap_or_else(|_| panic!("Expected cast to succeed for value {file_value}"));
        }
    }

    #[test]
    fn as_u8_roundtrip() {
        for i in 0..=7 {
            let file = File::try_from(i).expect("Expected valid file");
            assert_eq!(file.as_u8(), i);
        }
    }
}
