pub struct MoveCounter {
    halfmove: u128,
    fiftymove_rule: Vec<u128>,
}

impl MoveCounter {
    pub fn new() -> Self {
        Self {
            halfmove: 0,
            fiftymove_rule: vec![0],
        }
    }

    pub fn halfmove(&self) -> u128 {
        self.halfmove
    }

    pub fn fiftymove_rule(&self) -> u128 {
        *self.fiftymove_rule.last().unwrap()
    }

    pub fn tick(&mut self) {
        self.halfmove = self.halfmove.saturating_add(1);

        self.update_last_fiftymove_rule(self.fiftymove_rule().saturating_add(1));
    }

    pub fn untick(&mut self) {
        self.halfmove = self.halfmove.saturating_sub(1);

        let last_fiftymove_rule = self.fiftymove_rule();

        if last_fiftymove_rule > 0u128 {
            self.update_last_fiftymove_rule(last_fiftymove_rule.saturating_sub(1));
        } else if self.fiftymove_rule.len() > 1 {
            self.fiftymove_rule.pop();
        }
    }

    pub fn reset_fiftymove_rule(&mut self) {
        self.fiftymove_rule.push(0);
    }

    fn update_last_fiftymove_rule(&mut self, new_value: u128) {
        let last_fiftymove_rule = self.fiftymove_rule.last_mut().unwrap();
        *last_fiftymove_rule = new_value;
    }
}
