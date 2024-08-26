use crate::components::{piece_type::PieceType, team::Team};
use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    kind: PieceType,
    color: Team,
}

impl Piece {
    pub fn kind(&self) -> PieceType {
        self.kind
    }

    pub fn color(&self) -> Team {
        self.color
    }

    pub fn as_sentence(&self) -> String {
        format!("{} {}", self.color.as_sentence(), self.kind.as_sentence())
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.color, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{PieceType, Team};

    #[test]
    fn as_sentence() {
        for color in [Team::White, Team::Black] {
            for kind in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                let piece = Piece { kind, color };
                assert_eq!(
                    piece.as_sentence(),
                    format!("{} {}", color.as_sentence(), kind.as_sentence())
                );
            }
        }
    }

    #[test]
    fn display() {
        for color in [Team::White, Team::Black] {
            for kind in [
                PieceType::Pawn,
                PieceType::Knight,
                PieceType::Bishop,
                PieceType::Rook,
                PieceType::Queen,
                PieceType::King,
            ] {
                let piece = Piece { kind, color };
                assert_eq!(piece.to_string(), format!("{}{}", color, kind));
            }
        }
    }
}
