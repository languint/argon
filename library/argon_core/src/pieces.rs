pub enum Pieces {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Pieces {
    /// Attempt to construct a [`Piece`][Pieces] from a `char`
    /// 
    /// Not color sensitive, `p` or `P` will both be pawns.
    /// 
    /// # Returns
    /// Returns `None` if the `char` is not a piece.
    fn from_char(value: char) -> Option<Self> {
        match value {
            'p' => Some(Pieces::Pawn),
            'n' => Some(Pieces::Knight),
            'b' => Some(Pieces::Pawn),
            'r' => Some(Pieces::Rook),
            'q' => Some(Pieces::Queen),
            'k' => Some(Pieces::King),
            _ => None,
        }
    }
}