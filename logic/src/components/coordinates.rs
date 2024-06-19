use std::collections::HashMap;

pub struct Coordinates {
    row: usize,
    col: usize,
}

impl Coordinates {
    pub fn from(row: i32, col: i32) -> Option<Self> {
        if (0..8).contains(&row) && (0..8).contains(&col) {
            Some(Self {
                row: row as usize,
                col: col as usize,
            })
        } else {
            None
        }
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }

    pub fn as_algebraic(&self) -> String {
        let mappings: HashMap<usize, &str> = HashMap::from([
            (0, "a"),
            (1, "b"),
            (2, "c"),
            (3, "d"),
            (4, "e"),
            (5, "f"),
            (6, "g"),
            (7, "h"),
        ]);

        format!("{}{}", mappings.get(&self.col()).unwrap(), self.row() + 1)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from() {
        for row in 0..8 {
            for col in 0..8 {
                let coords = Coordinates::from(row, col).unwrap();
                assert_eq!(coords.row as i32, row);
                assert_eq!(coords.col as i32, col);
            }
        }

        assert!(Coordinates::from(-1, -1).is_none());
        assert!(Coordinates::from(-1, 0).is_none());
        assert!(Coordinates::from(0, -1).is_none());

        assert!(Coordinates::from(8, 8).is_none());
        assert!(Coordinates::from(8, 0).is_none());
        assert!(Coordinates::from(0, 8).is_none());

        assert!(Coordinates::from(-1, 8).is_none());
        assert!(Coordinates::from(8, -1).is_none());
    }

    #[test]
    fn row() {
        for row in 0..8 {
            for col in 0..8 {
                let coords = Coordinates{row, col};
                assert_eq!(coords.row(), row);
            }
        }
    }

    #[test]
    fn col() {
        for row in 0..8 {
            for col in 0..8 {
                let coords = Coordinates{row, col};
                assert_eq!(coords.col(), col);
            }
        }
    }

    #[test]
    fn as_algebraic() {
        let algebraic_cols = ["a", "b", "c", "d", "e", "f", "g", "h"];
        let algebraic_rows = ["1", "2", "3", "4", "5", "6", "7", "8"];

        for row in 0..8 {
            for col in 0..8 {
                let coords = Coordinates::from(row, col).unwrap();
                assert_eq!(
                    coords.as_algebraic(),
                    format!("{}{}", algebraic_cols[coords.col()], algebraic_rows[coords.row()])
                );
            }
        }
    }
}
