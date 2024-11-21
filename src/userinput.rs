use crate::{rules::*, Position};
use std::io;

pub fn await_rule() -> (Option<Rule>, usize) {
    println!("Enter a rule: (n) to cancel");

    let mut rule = String::new();
    io::stdin()
        .read_line(&mut rule)
        .expect("Failed to read line");
    let rule = rule.trim();
    println!("You entered: {}", rule);

    if rule == "n" {
        return (None, 0);
    }

    if let Some(permutation_rule) = parse_permutation_rule(rule) {
        return (Some(Rule::Permutation(permutation_rule)), 2);
    }

    if let Some(sum_rule) = parse_sum_rule(rule) {
        return (Some(Rule::Sum(sum_rule)), 2);
    }

    if let Some(relation_rule) = parse_relation_rule(rule) {
        return (Some(Rule::Relation(relation_rule)), 2);
    }

    println!("Invalid rule format, try again.");
    (None, 1)
}

fn parse_permutation_rule(input: &str) -> Option<PermutationRule> {
    let re = regex::Regex::new(r"^PERMUTATION \[([^\]]+)\]$").ok()?;
    let caps = re.captures(input)?;

    let positions_str = caps.get(1)?.as_str();
    let positions = parse_positions(positions_str)?;

    Some(PermutationRule { positions })
}

fn parse_sum_rule(input: &str) -> Option<SumRule> {
    let re = regex::Regex::new(r"^SUM \[([^\]]+)\] (\d+)$").ok()?;
    let caps = re.captures(input)?;

    let positions_str = caps.get(1)?.as_str();
    let positions = parse_positions(positions_str)?;

    let sum = caps.get(2)?.as_str().parse().ok()?;

    Some(SumRule { positions, sum })
}

fn parse_relation_rule(input: &str) -> Option<RelationRule> {
    let re = regex::Regex::new(r"^SMALLER (\d+),(\d+) (\d+),(\d+)$").ok()?;
    let caps = re.captures(input)?;

    let position1 = Position::new(
        caps.get(1)?.as_str().parse().ok()?,
        caps.get(2)?.as_str().parse().ok()?,
    )
    .unwrap();

    let position2 = Position::new(
        caps.get(3)?.as_str().parse().ok()?,
        caps.get(4)?.as_str().parse().ok()?,
    )
    .unwrap();

    Some(RelationRule {
        position1,
        position2,
    })
}

fn parse_positions(input: &str) -> Option<Vec<Position>> {
    let re = regex::Regex::new(r"(\d+),(\d+)").ok()?;
    let mut positions = Vec::new();

    for cap in re.captures_iter(input) {
        let row = cap.get(1)?.as_str().parse().ok()?;
        let col = cap.get(2)?.as_str().parse().ok()?;
        let position = Position::new(row, col)?;
        positions.push(position);
    }

    Some(positions)
}

/*
fn input_and_set_value() -> bool {
    println!("Enter row, column, and value (e.g., 1 1 1) or 'q' to quit:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim() == "q" {
        return false;
    }

    let parts: Vec<&str> = input.split_whitespace().collect();
    if parts.len() != 3 {
        println!("Invalid input. Please enter row, column, and value.");
        return true;
    }

    let pos = match Position::new(
        parts[0].parse().unwrap_or_default(),
        parts[1].parse().unwrap_or_default(),
    ) {
        Some(pos) => pos,
        None => {
            println!("Invalid row or column. Please enter valid numbers.");
            return true;
        }
    };

    let value: usize = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid value. Please enter a valid number.");
            return true;
        }
    };

    self.board.set_value(pos, value);
    true
}

*/
