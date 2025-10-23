use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
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

/// All [`Files`][`File`]
pub const ALL_FILES: [File; 8] = [
    File::A,
    File::B,
    File::C,
    File::D,
    File::E,
    File::F,
    File::G,
    File::H,
];

impl TryFrom<&char> for File {
    type Error = String;

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
            _ => Err(format!("Cannot parse `{value}` as a file!")),
        }
    }
}

impl TryFrom<u8> for File {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 7 {
            return Err(format!("Cannot parse {value} as a file!"));
        }
        Ok(unsafe { std::mem::transmute::<u8, File>(value) })
    }
}

impl From<File> for u8 {
    fn from(file: File) -> Self {
        file as u8
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", (*self as u8 + b'a') as char)
    }
}

impl File {
    /// Transmute a `u8` as a [`File`]
    ///
    /// # Safety
    /// This function is safe as long us `value` is less than `8`
    #[must_use]
    #[inline]
    pub unsafe fn from_u8(value: u8) -> Self {
        unsafe { std::mem::transmute(value) }
    }

    /// Get the `u8` representation of a [`File`]
    #[must_use]
    #[inline]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}
