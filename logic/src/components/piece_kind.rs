use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
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

impl PieceKind {
    pub fn as_sentence(&self) -> &str {
        match self {
            PieceKind::King => KING_LONG,
            PieceKind::Queen => QUEEN_LONG,
            PieceKind::Rook => ROOK_LONG,
            PieceKind::Bishop => BISHOP_LONG,
            PieceKind::Knight => KNIGHT_LONG,
            PieceKind::Pawn => PAWN_LONG,
        }
    }
}

const KING_SHORT: &str = "k";
const QUEEN_SHORT: &str = "q";
const ROOK_SHORT: &str = "r";
const BISHOP_SHORT: &str = "b";
const KNIGHT_SHORT: &str = "n";
const PAWN_SHORT: &str = "p";

impl fmt::Display for PieceKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PieceKind::King => write!(f, "{}", KING_SHORT),
            PieceKind::Queen => write!(f, "{}", QUEEN_SHORT),
            PieceKind::Rook => write!(f, "{}", ROOK_SHORT),
            PieceKind::Bishop => write!(f, "{}", BISHOP_SHORT),
            PieceKind::Knight => write!(f, "{}", KNIGHT_SHORT),
            PieceKind::Pawn => write!(f, "{}", PAWN_SHORT),
        }
    }
}
