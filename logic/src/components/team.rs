pub enum Team {
    White,
    Black,
}

impl ToString for Team {
    fn to_string(&self) -> String {
        match self {
            Team::White => String::from("w"),
            Team::Black => String::from("b"),
        }
    }
}
