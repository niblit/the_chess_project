use std::fmt;

#[derive(Clone, Copy, PartialEq, Eq)]
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
