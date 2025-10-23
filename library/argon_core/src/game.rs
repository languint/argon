use crate::{board::Board, castling::CastlingRights, chess_move::Move, color::Color};

pub struct Game {
    pub board: Board,
    pub color_to_move: Color,
    pub moves: Vec<Move>,
    pub castling_rights: CastlingRights,
}
