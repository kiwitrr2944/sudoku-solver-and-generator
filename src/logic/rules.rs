use super::board::{Board, Position};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Rule {
    Sum(SumRule),
    Permutation(PermutationRule),
    Relation(RelationRule),
}

#[derive(Deserialize, Serialize)]
pub enum RuleCheckResult {
    Critical(String),
    Unfulfilled(String),
    Ok,
}

impl Rule {
    pub fn check(&self, board: &Board) -> RuleCheckResult {
        match self {
            Rule::Sum(r) => r.check(board),
            Rule::Permutation(r) => r.check(board),
            Rule::Relation(r) => r.check(board),
        }
    }

    pub fn get_positions(&self) -> Vec<Position> {
        match self {
            Rule::Sum(r) => r.positions.clone(),
            Rule::Permutation(r) => r.positions.clone(),
            Rule::Relation(r) => r.positions.clone(),
        }
    }

    pub fn add_position(&mut self, pos: Position) {
        match self {
            Rule::Sum(r) => r.positions.push(pos),
            Rule::Permutation(r) => r.positions.push(pos),
            Rule::Relation(r) => r.positions.push(pos),
        }
    }

    pub fn remove_position(&mut self, pos: Position) {
        match self {
            Rule::Sum(r) => r.positions.retain(|&x| x != pos),
            Rule::Permutation(r) => r.positions.retain(|&x| x != pos),
            Rule::Relation(r) => r.positions.retain(|&x| x != pos),
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Rule::Sum(r) => r.index,
            Rule::Permutation(r) => r.index,
            Rule::Relation(r) => r.index,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct SumRule {
    pub positions: Vec<Position>,
    pub sum: usize,
    index: usize,
}

impl SumRule {
    pub fn new(positions: Vec<Position>, sum: usize, index: usize) -> Self {
        SumRule {
            positions,
            sum,
            index,
        }
    }

    pub fn check(&self, board: &Board) -> RuleCheckResult {
        let current_sum: usize = self
            .positions
            .iter()
            .map(|&pos| board.get_value(pos))
            .sum();

        match current_sum.cmp(&self.sum) {
            std::cmp::Ordering::Less => RuleCheckResult::Unfulfilled(format!(
                "(sum): positions {:?} should sum to {}, currently {}",
                self.positions, self.sum, current_sum
            )),
            std::cmp::Ordering::Greater => RuleCheckResult::Critical(format!(
                "(sum): positions {:?} should sum to {}, currently {}",
                self.positions, self.sum, current_sum
            )),
            std::cmp::Ordering::Equal => RuleCheckResult::Ok,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PermutationRule {
    pub positions: Vec<Position>,
    index: usize,
}

impl PermutationRule {
    pub fn new(positions: Vec<Position>, index: usize) -> Self {
        PermutationRule { positions, index }
    }
    pub fn check(&self, board: &Board) -> RuleCheckResult {
        if (self.positions.len()) != board.get_side() {
            return RuleCheckResult::Ok;
        }
        let mut values: Vec<usize> = self
            .positions
            .iter()
            .map(|&pos| board.get_value(pos))
            .filter(|&x| x > 0)
            .collect();

        values.sort();
        let mut unique_values = values.clone();
        unique_values.dedup();

        if unique_values.len() != values.len() {
            RuleCheckResult::Critical(format!(
                "(permutation): positions {:?} should be a permutation",
                self.positions
            ))
        } else if values.len() < board.get_side() {
            RuleCheckResult::Unfulfilled(format!(
                "(permutation): positions {:?} should be a permutation, (elements are missing)",
                self.positions
            ))
        } else if unique_values.first() != Some(&1)
            || unique_values.last() != Some(&board.get_side())
        {
            RuleCheckResult::Critical(format!(
                "(permutation): positions {:?} should be a permutation",
                self.positions
            ))
        } else {
            RuleCheckResult::Ok
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RelationRule {
    pub positions: Vec<Position>,
    index: usize,
}

impl RelationRule {
    pub fn new(index: usize) -> Self {
        RelationRule {
            positions: vec![],
            index,
        }
    }

    pub fn check(&self, board: &Board) -> RuleCheckResult {
        if self.positions.len() != 2 {
            return RuleCheckResult::Ok;
        }
        let value1 = board.get_value(self.positions[0]);
        let value2 = board.get_value(self.positions[1]);
        if value1 == 0 || value2 == 0 {
            RuleCheckResult::Unfulfilled(format!(
                "(relation): position {:?} or {:?} not filled",
                self.positions[0], self.positions[1]
            ))
        } else if value1 >= value2 {
            RuleCheckResult::Critical(format!(
                "(relation): position {:?} should be less than {:?}",
                self.positions[0], self.positions[1]
            ))
        } else {
            RuleCheckResult::Ok
        }
    }
}
