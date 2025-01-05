use super::board::{Board, Position};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
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

#[derive(Deserialize, Serialize, Clone)]
pub struct SumRule {
    pub positions: Vec<Position>,
    pub sum: usize,
}

impl SumRule {
    pub fn check(&self, board: &Board) -> RuleCheckResult {
        let current_sum: usize = self
            .positions
            .iter()
            .map(|&pos| board.get_value(Some(pos)).unwrap_or_default())
            .sum();

        match current_sum.cmp(&self.sum) {
            std::cmp::Ordering::Less => RuleCheckResult::Unfulfilled(format!(
                "RULE (sum): positions {:?} should sum to {}, currently {}",
                self.positions, self.sum, current_sum
            )),
            std::cmp::Ordering::Greater => RuleCheckResult::Critical(format!(
                "RULE (sum): positions {:?} should sum to {}, currently {}",
                self.positions, self.sum, current_sum
            )),
            std::cmp::Ordering::Equal => RuleCheckResult::Ok,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct PermutationRule {
    pub positions: Vec<Position>,
}

impl PermutationRule {
    pub fn check(&self, board: &Board) -> RuleCheckResult {
        let mut values: Vec<usize> = self
            .positions
            .iter()
            .filter_map(|&pos| board.get_value(Some(pos)))
            .collect();

        values.sort();

        let mut unique_values = values.clone();
        unique_values.dedup();

        if unique_values.len() != values.len() || unique_values.first() != Some(&1) || unique_values.last() != Some(&board.get_side()) {
            RuleCheckResult::Critical(format!(
                "RULE (permutation): positions {:?} should be a permutation",
                self.positions
            ))
        } else if unique_values.len() < board.get_side() {
            RuleCheckResult::Unfulfilled(format!(
                "RULE (permutation): positions {:?} should be a permutation, (elements are missing)",
                self.positions
            ))
        } else {
            RuleCheckResult::Ok
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct RelationRule {
    pub position1: Position,
    pub position2: Position,
}

impl RelationRule {
    pub fn check(&self, board: &Board) -> RuleCheckResult {
        let value1 = board.get_value(Some(self.position1));
        let value2 = board.get_value(Some(self.position2));
        if value1.is_none() || value2.is_none() {
            RuleCheckResult::Unfulfilled(format!(
                "RULE (relation): position {:?} or {:?} not filled",
                self.position1, self.position2
            ))
        } else if value1.unwrap() >= value2.unwrap() {
            RuleCheckResult::Critical(format!(
                "RULE (relation): position {:?} should be less than {:?}",
                self.position1, self.position2
            ))
        } else {
            RuleCheckResult::Ok
        }
    }
}
