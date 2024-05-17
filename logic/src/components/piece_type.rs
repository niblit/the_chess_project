use std::fmt;

pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

const KING_LONG: &str = "King";
const QUEEN_LONG: &str = "Queen";
const ROOK_LONG: &str = "Rook";
const BISHOP_LONG: &str = "Bishop";
const KNIGHT_LONG: &str = "Knight";
const PAWN_LONG: &str = "Pawn";

impl PieceType {
    pub fn as_sentence(&self) -> &str {
        match self {
            PieceType::King => KING_LONG,
            PieceType::Queen => QUEEN_LONG,
            PieceType::Rook => ROOK_LONG,
            PieceType::Bishop => BISHOP_LONG,
            PieceType::Knight => KNIGHT_LONG,
            PieceType::Pawn => PAWN_LONG,
        }
    }
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
