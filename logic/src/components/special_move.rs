use super::piece::Piece;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SpecialMove {
    Castle,
    EnPassant,
    PawnPromotion(Piece)
}
