use crate::components::{piece_type::PieceType, team::Team};
use std::fmt;

pub struct Piece {
    kind: PieceType,
    color: Team,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color, self.kind)
    }
}
