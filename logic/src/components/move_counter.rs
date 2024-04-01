pub struct MoveCounter {
    halfmove: u128,
    fiftymove_rule: u128,
}

impl MoveCounter {
    pub fn new() -> Self {
        Self {
            halfmove: 0,
            fiftymove_rule: 0,
        }
    }

    pub fn halfmove(&self) -> u128 {
        self.halfmove
    }

    pub fn fiftymove_rule(&self) -> u128 {
        self.fiftymove_rule
    }

    pub fn tick(&mut self) {
        self.halfmove += 1;
        self.fiftymove_rule += 1;
    }

    pub fn untick(&mut self) {
        self.halfmove.saturating_sub(1);
        self.fiftymove_rule.saturating_sub(1);
    }

    pub fn reset_fifty_rule_count(&mut self) {
        self.fiftymove_rule = 0;
    }
}
