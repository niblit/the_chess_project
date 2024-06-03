#[derive(Clone, Copy)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }

    pub fn custom_start(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> Self {
        Self {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        }
    }

    pub fn get_white_king_side(&self) -> bool {
        self.white_king_side
    }

    pub fn remove_white_king_side(&mut self) {
        self.white_king_side = false;
    }

    pub fn get_white_queen_side(&self) -> bool {
        self.white_queen_side
    }

    pub fn remove_white_queen_side(&mut self) {
        self.white_queen_side = false;
    }

    pub fn get_black_king_side(&self) -> bool {
        self.black_king_side
    }

    pub fn remove_black_king_side(&mut self) {
        self.black_king_side = false;
    }

    pub fn get_black_queen_side(&self) -> bool {
        self.black_queen_side
    }

    pub fn remove_black_queen_side(&mut self) {
        self.black_queen_side = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_castling_rights() {
        let rights = CastlingRights::new();
        assert!(rights.get_white_king_side());
        assert!(rights.get_white_queen_side());
        assert!(rights.get_black_king_side());
        assert!(rights.get_black_queen_side());
    }

    #[test]
    fn test_custom_castling_rights() {
        let rights = CastlingRights::custom_start(true, false, false, true);
        assert!(rights.get_white_king_side());
        assert!(!rights.get_white_queen_side());
        assert!(!rights.get_black_king_side());
        assert!(rights.get_black_queen_side());
    }

    #[test]
    fn test_remove_rights() {
        let mut rights = CastlingRights::new();
        rights.remove_white_king_side();

        assert!(!rights.get_white_king_side());
        assert!(rights.get_white_queen_side());
        assert!(rights.get_black_king_side());
        assert!(rights.get_black_queen_side());

        rights.remove_white_queen_side();

        assert!(!rights.get_white_king_side());
        assert!(!rights.get_white_queen_side());
        assert!(rights.get_black_king_side());
        assert!(rights.get_black_queen_side());

        rights.remove_black_king_side();

        assert!(!rights.get_white_king_side());
        assert!(!rights.get_white_queen_side());
        assert!(!rights.get_black_king_side());
        assert!(rights.get_black_queen_side());

        rights.remove_black_queen_side();

        assert!(!rights.get_white_king_side());
        assert!(!rights.get_white_queen_side());
        assert!(!rights.get_black_king_side());
        assert!(!rights.get_black_queen_side());
    }
}
