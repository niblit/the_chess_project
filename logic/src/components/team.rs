use std::fmt;

#[derive(Clone, Copy)]
pub enum Team {
    Black,
    White,
}

const BLACK_LONG: &str = "Black";
const WHITE_LONG: &str = "White";

impl Team {
    pub fn as_sentence(&self) -> &str {
        match self {
            Team::Black => BLACK_LONG,
            Team::White => WHITE_LONG,
        }
    }
}

const BLACK_SHORT: &str = "b";
const WHITE_SHORT: &str = "w";

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Team::Black => write!(f, "{}", BLACK_SHORT),
            Team::White => write!(f, "{}", WHITE_SHORT),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn as_sentence() {
        assert_eq!(Team::Black.as_sentence(), BLACK_LONG);
        assert_eq!(Team::White.as_sentence(), WHITE_LONG);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", Team::Black), BLACK_SHORT);
        assert_eq!(format!("{}", Team::White), WHITE_SHORT);
    }
}
