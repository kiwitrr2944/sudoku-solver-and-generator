use super::board::{Board, Position};
use super::rules::{self, PermutationRule, Rule};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    board: Board,
    rules: Vec<Rule>,
}

impl Game {
    pub fn new(side: usize, sub_rows: usize, sub_cols: usize) -> Self {
        let mut game = Game {
            board: Board::new(),
            rules: Vec::new(),
        };

        for row in 0..side {
            let positions: Vec<Position> = (0..side)
                .filter_map(|col| Position::new(row + 1, col + 1))
                .collect();
            game.add_rule(Rule::Permutation(PermutationRule { positions }));
        }

        for col in 0..side {
            let positions: Vec<Position> = (0..side)
                .filter_map(|row| Position::new(row + 1, col + 1))
                .collect();
            game.add_rule(Rule::Permutation(PermutationRule { positions }));
        }

        for sub_row in 0..(side / sub_rows) {
            for sub_col in 0..(side / sub_cols) {
                let mut positions = Vec::new();
                for row in 0..sub_rows {
                    for col in 0..sub_cols {
                        let pos_row = sub_row * sub_rows + row + 1;
                        let pos_col = sub_col * sub_cols + col + 1;
                        if let Some(pos) = Position::new(pos_row, pos_col) {
                            positions.push(pos);
                        }
                    }
                }
                game.add_rule(Rule::Permutation(PermutationRule { positions }));
            }
        }

        game
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn check_rules(&self) -> (Option<Vec<String>>, Option<Vec<String>>) {
        let mut violations = Vec::new();
        let mut pending: Vec<String> = Vec::new();

        for rule in &self.rules {
            let result = match rule {
                Rule::Sum(r) => r.check(&self.board),
                Rule::Permutation(r) => r.check(&self.board),
                Rule::Relation(r) => r.check(&self.board),
            };

            match result {
                rules::RuleCheckResult::Critical(msg) => {
                    violations.push(msg);
                }
                rules::RuleCheckResult::Unfulfilled(msg) => {
                    pending.push(msg);
                }
                rules::RuleCheckResult::Ok => {}
            }
        }

        (
            if violations.is_empty() { None } else { Some(violations) },
            if pending.is_empty() { None } else { Some(pending) },
        )
    }

    pub fn save_to_file(&self, filename: &str) {
        let path = Path::new(filename);
        let serialized = serde_json::to_string(&self).unwrap();
        dbg!("Saving to file");
        let _ = fs::write(path, serialized);
    }

    pub fn load_from_file(filename: &str) -> Self {
        let path = Path::new(filename);
        let serialized = fs::read_to_string(path).unwrap();
        serde_json::from_str(&serialized).unwrap()
    }

    pub fn display(&self) {
        self.board.display();
    }

    pub fn board(&self) -> Board {
        self.board.clone()
    }

    pub fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    pub fn set_value(&mut self, pos: Option<Position>, value: usize) {
        dbg!("set_value", pos, value);
        match pos {
            Some(pos) => self.board.set_value(pos, value),
            None => {}
        }
    }
    pub fn get_value(&self, pos: Option<Position>) -> Option<usize> {
        self.board.get_value(pos)
    }
}
