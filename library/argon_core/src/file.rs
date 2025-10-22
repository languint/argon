pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl TryFrom<&char> for File {
    type Error = String;
    /// Attempt to construct a [`File`] from a `char`
    /// 
    /// Not case sensitive, `a` or `A` will both be [`File::A`]
    /// 
    /// # Errors
    /// Returns `Err` if the file couldn't be parsed
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        match value.to_ascii_lowercase() {
            'a' => Ok(File::A),
            'b' => Ok(File::B),
            'c' => Ok(File::C),
            'd' => Ok(File::D),
            'e' => Ok(File::E),
            'f' => Ok(File::F),
            'g' => Ok(File::G),
            'h' => Ok(File::H),
            _ => Err(format!("Cannot parse `{value}` as a file!"))
        }
    }
}
