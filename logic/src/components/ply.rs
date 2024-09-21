use super::{coordinates::Coordinates, piece::Piece, special_move::SpecialMove, square::SquareType};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Ply {
    start: Coordinates,
    end: Coordinates,

    piece_moved: Piece,
    piece_captured: SquareType,

    special_move: Option<SpecialMove>,
}

impl Ply {
    pub fn get_id(&self) -> String {
        let id_start = format!("{}{}", self.piece_moved, self.start.as_algebraic());
        let id_capture = if self.piece_captured.is_some() {String::from("x")} else {String::new()};
        let id_end = format!("{}{}", if let Some(end_piece) = self.piece_captured {end_piece.to_string()} else {String::new()}, self.end.as_algebraic());
        let id_special_move = if let Some(sm) = &self.special_move {
            match sm {
                SpecialMove::Castle => String::from("[Castle]"),
                SpecialMove::EnPassant => String::from("[e.p.]"),
                SpecialMove::PawnPromotion(piece) => format!("[={}]", piece.kind()),
            }
        }
        else {
            String::new()
        };

        let id = format!("{}{}{}{}", id_start, id_capture, id_end, id_special_move);
        id
    }

    pub fn new(start: Coordinates, end: Coordinates, piece_moved: Piece, piece_captured: SquareType, special_move: Option<SpecialMove>) -> Self {
        Self {
            start,
            end,

            piece_moved,
            piece_captured,

            special_move,
        }
    }
}
