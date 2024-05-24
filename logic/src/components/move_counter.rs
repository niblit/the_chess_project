pub struct MoveCounter {
    halfmove: u128,
    irreversible_move: Vec<u128>,
}

impl MoveCounter {
    pub fn new() -> Self {
        Self {
            halfmove: 0,
            irreversible_move: vec![0],
        }
    }

    pub fn custom_start(halfmove: u128, irreversible_move: u128) -> Self {
        Self {
            halfmove,
            irreversible_move: vec![irreversible_move],
        }
    }

    pub fn get_halfmove(&self) -> u128 {
        self.halfmove
    }

    pub fn is_fifty_move_rule(&self) -> bool {
        self.get_last_irreversible_move() >= 100
    }

    pub fn tick(&mut self, was_reversible_move_made: bool) {
        self.increment_halfmove();
        if was_reversible_move_made {
            self.reversible_move_made();
        } else {
            self.increment_irreversible_move();
        }
    }

    pub fn untick(&mut self) {
        self.decrement_halfmove();
        self.decrement_irreversible_move();
    }

    fn increment_halfmove(&mut self) {
        self.halfmove = self.halfmove.saturating_add(1);
    }

    fn decrement_halfmove(&mut self) {
        self.halfmove = self.halfmove.saturating_sub(1);
    }

    fn reversible_move_made(&mut self) {
        self.irreversible_move.push(0);
    }

    fn increment_irreversible_move(&mut self) {
        let last = self.irreversible_move.last_mut().unwrap();
        *last = last.saturating_add(1);
    }

    fn decrement_irreversible_move(&mut self) {
        if self.get_last_irreversible_move() > 0 {
            let last = self.irreversible_move.last_mut().unwrap();
            *last = last.saturating_sub(1);
        } else if self.get_last_irreversible_move() == 0 && self.irreversible_move.len() > 1 {
            self.irreversible_move.pop();
        }
    }

    fn get_last_irreversible_move(&self) -> u128 {
        *self.irreversible_move.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let counter = MoveCounter::new();
        assert_eq!(counter.halfmove, 0);
        assert_eq!(counter.irreversible_move, vec![0]);
    }

    #[test]
    fn custom_start() {
        let counter = MoveCounter::custom_start(3, 1416);
        assert_eq!(counter.halfmove, 3);
        assert_eq!(counter.irreversible_move, vec![1416]);
    }

    #[test]
    fn get_halfmove() {
        let counter = MoveCounter::new();
        assert_eq!(counter.get_halfmove(), counter.halfmove);

        let counter = MoveCounter::custom_start(45, 294);
        assert_eq!(counter.get_halfmove(), counter.halfmove);
    }

    #[test]
    fn is_fifty_move_rule() {
        let counter = MoveCounter::new();
        assert!(!counter.is_fifty_move_rule());

        let counter = MoveCounter::custom_start(150, 50);
        assert!(!counter.is_fifty_move_rule());

        let counter = MoveCounter::custom_start(100, 100);
        assert!(counter.is_fifty_move_rule());

        let counter = MoveCounter::custom_start(50, 150);
        assert!(counter.is_fifty_move_rule());
    }

    #[test]
    fn tick() {
        let mut counter = MoveCounter::new();
        for _ in 0..16 {
            counter.tick(false);
        }

        for _ in 0..4 {
            counter.tick(true);
        }

        counter.tick(false);

        assert_eq!(counter.halfmove, 21);
        assert_eq!(counter.irreversible_move, vec![16, 0, 0, 0, 1]);
    }

    #[test]
    fn untick() {
        let mut counter = MoveCounter {
            halfmove: 32,
            irreversible_move: vec![16, 4, 6, 6],
        };

        for _ in 0..16 {
            counter.untick();
        }

        assert_eq!(counter.halfmove, 16);
        assert_eq!(counter.irreversible_move, vec![16, 2]);
    }

    #[test]
    fn increment_halfmove() {
        let mut counter = MoveCounter::new();
        for _ in 0..64 {
            counter.increment_halfmove();
        }

        assert_eq!(counter.halfmove, 64);
    }

    #[test]
    fn decrement_halfmove() {
        let mut counter = MoveCounter {
            halfmove: 64,
            irreversible_move: vec![0],
        };

        for _ in 0..32 {
            counter.decrement_halfmove();
        }

        assert_eq!(counter.halfmove, 32);

        for _ in 0..256 {
            counter.decrement_halfmove();
        }

        assert_eq!(counter.halfmove, 0);
    }

    #[test]
    fn reversible_move_made() {
        let mut counter = MoveCounter::new();
        for _ in 0..16 {
            counter.increment_irreversible_move();
        }

        counter.reversible_move_made();
        counter.reversible_move_made();

        for _ in 0..32 {
            counter.increment_irreversible_move();
        }

        counter.reversible_move_made();

        for _ in 0..64 {
            counter.increment_irreversible_move();
        }

        assert_eq!(counter.irreversible_move, vec![16, 0, 32, 64]);
    }

    #[test]
    fn increment_irreversible_move() {
        let mut counter = MoveCounter::new();
        for _ in 0..64 {
            counter.increment_irreversible_move();
        }

        assert_eq!(counter.irreversible_move, vec![64]);
    }

    #[test]
    fn decrement_irreversible_move() {
        let mut counter = MoveCounter {
            halfmove: 64,
            irreversible_move: vec![12, 0, 0, 234, 0, 32, 2],
        };

        for _ in 0..40 {
            counter.decrement_irreversible_move();
        }

        assert_eq!(counter.irreversible_move, vec![12, 0, 0, 231]);

        for _ in 0..240 {
            counter.decrement_irreversible_move();
        }

        assert_eq!(counter.irreversible_move, vec![6]);

        for _ in 0..1024 {
            counter.decrement_irreversible_move();
        }
        assert_eq!(counter.irreversible_move, vec![0]);
    }

    #[test]
    fn get_last_irreversible_move() {
        let counter = MoveCounter {
            halfmove: 21,
            irreversible_move: vec![0],
        };
        assert_eq!(counter.get_last_irreversible_move(), 0);

        let counter = MoveCounter {
            halfmove: 21,
            irreversible_move: vec![123, 34, 231, 0, 23],
        };
        assert_eq!(counter.get_last_irreversible_move(), 23);
    }
}
