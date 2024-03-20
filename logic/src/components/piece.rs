use crate::components::{piece_type::PieceType, team::Team};

pub struct Piece {
    kind: PieceType,
    color: Team,
}

impl ToString for Piece {
    fn to_string(&self) -> String {
        format!("{}{}", self.color.to_string(), self.kind.to_string())
    }
}
