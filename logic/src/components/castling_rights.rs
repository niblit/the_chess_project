pub struct CastlingRights {
    WhiteKingSide: bool,
    WhiteQueenSide: bool,
    BlackKingSide: bool,
    BlackQueenSide: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            WhiteKingSide: true,
            WhiteQueenSide: true,
            BlackKingSide: true,
            BlackQueenSide: true,
        }
    }

    pub fn get_white_king_side(&self) -> bool {
        self.WhiteKingSide
    }

    pub fn remove_white_king_side(&mut self) {
        self.WhiteKingSide = false;
    }

    pub fn get_white_queen_side(&self) -> bool {
        self.WhiteQueenSide
    }

    pub fn remove_white_queen_side(&mut self) {
        self.WhiteQueenSide = false;
    }

    pub fn get_black_king_side(&self) -> bool {
        self.BlackKingSide
    }

    pub fn remove_black_king_side(&mut self) {
        self.BlackKingSide = false;
    }

    pub fn get_black_queen_side(&self) -> bool {
        self.BlackQueenSide
    }

    pub fn remove_black_queen_side(&mut self) {
        self.BlackQueenSide = false;
    }
}
