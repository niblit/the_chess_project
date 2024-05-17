use std::fmt;

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

const KING_SHORT: &str = "k";
const QUEEN_SHORT: &str = "q";
const ROOK_SHORT: &str = "r";
const BISHOP_SHORT: &str = "b";
const KNIGHT_SHORT: &str = "n";
const PAWN_SHORT: &str = "p";

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceType::King => write!(f, "{}", KING_SHORT),
            PieceType::Queen => write!(f, "{}", QUEEN_SHORT),
            PieceType::Rook => write!(f, "{}", ROOK_SHORT),
            PieceType::Bishop => write!(f, "{}", BISHOP_SHORT),
            PieceType::Knight => write!(f, "{}", KNIGHT_SHORT),
            PieceType::Pawn => write!(f, "{}", PAWN_SHORT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", PieceType::King), KING_SHORT);
        assert_eq!(format!("{}", PieceType::Queen), QUEEN_SHORT);
        assert_eq!(format!("{}", PieceType::Rook), ROOK_SHORT);
        assert_eq!(format!("{}", PieceType::Bishop), BISHOP_SHORT);
        assert_eq!(format!("{}", PieceType::Knight), KNIGHT_SHORT);
        assert_eq!(format!("{}", PieceType::Pawn), PAWN_SHORT);
    }
}
