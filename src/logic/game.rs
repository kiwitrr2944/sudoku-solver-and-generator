use super::board::{Board, Position};
use super::rules::{self, PermutationRule, Rule};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct Game {
    board: Board,
    rules: Vec<Rule>,
    base_rule_count: usize,
}

impl Game {
    pub fn new(side: usize, sub_rows: usize, sub_cols: usize) -> Self {
        let mut game = Game {
            board: Board::new(side),
            rules: Vec::new(),
            base_rule_count: 0,
        };

        let mut rc = 0;
        for row in 1..=side {
            let positions: Vec<Position> = (1..=side)
                .filter_map(|col| Position::new(row, col))
                .collect();
            game.add_rule(Rule::Permutation(PermutationRule::new(positions, rc)));
            rc += 1;
        }
        
        for col in 1..=side {
            let positions: Vec<Position> = (1..=side)
            .filter_map(|row| Position::new(row, col))
            .collect();
            game.add_rule(Rule::Permutation(PermutationRule::new(positions, rc)));
            rc += 1;
        }
    
        for sub_row in 0..side/sub_rows {
            for sub_col in 0..side/sub_cols {
                dbg!(sub_row, sub_col, "----------");
                let mut positions = Vec::new();
                for row in 1..=sub_rows {
                    for col in 1..=sub_cols {
                        let pos_row = sub_row * sub_rows + row;
                        let pos_col = sub_col * sub_cols + col;
                        if let Some(pos) = Position::new(pos_row, pos_col) {
                            positions.push(pos);
                        }
                    }
                }
                game.add_rule(Rule::Permutation(PermutationRule::new(positions, rc)));
                rc += 1;
            }
        }
        
        dbg!(&game.rules);
        game.base_rule_count = rc;
        game
    }

    pub fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    pub fn add_position_to_rule(&mut self, index: usize, pos: Position) {
        self.rules[index + self.base_rule_count].add_position(pos);
    }

    pub fn remove_position_from_rule(&mut self, index: usize, pos: Position) {
        self.rules[index + self.base_rule_count].remove_position(pos);
    }

    pub fn get_rule(&self, index: usize) -> Rule {
        self.rules[index + self.base_rule_count].clone()
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
        print!("{}", self.board()); 
    }

    pub fn board(&self) -> Board {
        self.board.clone()
    }

    pub fn rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

    pub fn set_value(&mut self, pos: Option<Position>, value: usize) {
        print!("{}", self.board());
        match pos {
            Some(pos) => self.board.set_value(pos, value),
            None => {}
        }
    }
    pub fn get_value(&self, pos: Option<Position>) -> usize {
        self.board.get_value(pos)
    }

    pub fn get_side(&self) -> usize {
        self.board.get_side()
    }

    pub fn get_base_rule_count(&self) -> usize {
        self.base_rule_count
    }

    pub fn set_board(&mut self, board: Board) {
        self.board = board;
    }

    pub fn get_rules_state(&self) -> String {
        let (violations, pendings) = self.check_rules();
        let mut state = String::from("Violated rules: \n");
        if let Some(violations) = violations {
            for violation in violations {
                state += &violation;
                state += "\n";
            }
        }

        state += "Pending rules: \n";

        if let Some(pendings) = pendings {
            for pending in pendings {
                state += &pending;
                state += "\n";
            }
        }
        state
    }
}
