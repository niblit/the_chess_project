pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

impl ToString for PieceType {
    fn to_string(&self) -> String {
        match self {
            PieceType::King => String::from("k"),
            PieceType::Queen => String::from("q"),
            PieceType::Rook => String::from("r"),
            PieceType::Bishop => String::from("b"),
            PieceType::Knight => String::from("n"),
            PieceType::Pawn => String::from("p"),
        }
    }
}
