use super::{coordinates::Coordinates, piece::Piece, square::SquareType};

pub struct Board {
    grid: [[SquareType; 8]; 8],
}

impl Board {
    pub const fn new() -> Self {
        use super::{piece::Piece, piece_kind::PieceKind::*, team::Team::*};
        Self {
            grid: [
                [
                    Some(Piece::new(Rook, Black)),
                    Some(Piece::new(Knight, Black)),
                    Some(Piece::new(Bishop, Black)),
                    Some(Piece::new(Queen, Black)),
                    Some(Piece::new(King, Black)),
                    Some(Piece::new(Bishop, Black)),
                    Some(Piece::new(Knight, Black)),
                    Some(Piece::new(Rook, Black)),
                ],
                [Some(Piece::new(Pawn, Black)); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece::new(Pawn, White)); 8],
                [
                    Some(Piece::new(Rook, White)),
                    Some(Piece::new(Knight, White)),
                    Some(Piece::new(Bishop, White)),
                    Some(Piece::new(Queen, White)),
                    Some(Piece::new(King, White)),
                    Some(Piece::new(Bishop, White)),
                    Some(Piece::new(Knight, White)),
                    Some(Piece::new(Rook, White)),
                ],
            ],
        }
    }

    pub fn get_pieces_on_board(&self) -> Vec<(Coordinates, Piece)> {
        let mut pieces_on_board = Vec::new();
        for row in 0..8 {
            for col in 0..8 {
                if let Some(piece) = self.grid[row][col] {
                    let coordinates = Coordinates::from_usize(row, col).unwrap();
                    pieces_on_board.push((coordinates, piece));
                }
            }
        }
        pieces_on_board
    }

    pub fn get(&self, coordinates: Coordinates) -> SquareType {
        self.grid[coordinates.row()][coordinates.col()]
    }

    pub fn set(&mut self, coordinates: Coordinates, piece: Piece) {
        self.grid[coordinates.row()][coordinates.col()] = Some(piece);
    }

    pub fn clear(&mut self, coordinates: Coordinates) {
        self.grid[coordinates.row()][coordinates.col()] = None;
    }
}
