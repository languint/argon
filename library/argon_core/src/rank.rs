pub struct Rank(u8);

impl TryFrom<&char> for Rank {
    type Error = String;

    /// Attempt to construct a [`Rank`] from a `char`.
    ///
    /// Accepts digits `'1'` through `'8'`.
    ///
    /// # Errors
    /// Returns `Err` if the rank couldn't be parsed.
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value {
            '1' => Ok(Rank(1)),
            '2' => Ok(Rank(2)),
            '3' => Ok(Rank(3)),
            '4' => Ok(Rank(4)),
            '5' => Ok(Rank(5)),
            '6' => Ok(Rank(6)),
            '7' => Ok(Rank(7)),
            '8' => Ok(Rank(8)),
            _ => Err(format!("Cannot parse `{value}` as a rank!")),
        }
    }
}
