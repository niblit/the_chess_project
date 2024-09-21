use crate::components::{piece_kind::PieceKind, team::Team};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    kind: PieceKind,
    color: Team,
}

impl Piece {
    pub const fn new(kind: PieceKind, color: Team) -> Self {
        Self { kind, color }
    }

    pub fn kind(&self) -> PieceKind {
        self.kind
    }

    pub fn color(&self) -> Team {
        self.color
    }

    pub fn as_sentence(&self) -> String {
        format!("{} {}", self.color.as_sentence(), self.kind.as_sentence())
    }
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.color, self.kind)
    }
}
