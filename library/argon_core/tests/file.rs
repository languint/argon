#[cfg(test)]
mod file_tests {
    use std::ops::Range;

    use argon_core::file::File;

    #[test]
    fn from_char() {
        let alphabet = String::from_utf8((b'a'..=b'h').chain(b'A'..=b'H').collect())
            .expect("Expected string building to succeed");

        for char in alphabet.chars() {
            File::try_from(&char).expect("Expected cast to succeed");
        }
    }

    #[test]
    fn from_u8() {
        let numbers: Range<u8> = 0..7;

        for file in numbers.into_iter() {
            File::try_from(file).expect("Expected cast to succeed");
        }
    }
}
