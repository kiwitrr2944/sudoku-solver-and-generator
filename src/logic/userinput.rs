use crate::{board::Position, rules::*};
use std::io;

pub fn await_rule() -> (Option<Rule>, usize) {
    println!("Enter a rule: (p) to play (h) for help");

    let mut rule = String::new();
    io::stdin()
        .read_line(&mut rule)
        .expect("Failed to read line");
    let rule = rule.trim();

    println!("You entered: {}", rule);

    match rule {
        "p" => (None, 0),
        "h" => {
            println!("Rule format examples:");
            println!("PERMUTATION [1,1 2,2 3,3 4,4]");
            println!("SUM [1,1 1,2] 7");
            println!("SMALLER 1,1 1,2");
            (None, 1)
        }
        _ => {
            if let Some(permutation_rule) = parse_permutation_rule(rule) {
                (Some(Rule::Permutation(permutation_rule)), 2)
            } else if let Some(sum_rule) = parse_sum_rule(rule) {
                (Some(Rule::Sum(sum_rule)), 2)
            } else if let Some(relation_rule) = parse_relation_rule(rule) {
                (Some(Rule::Relation(relation_rule)), 2)
            } else {
                println!("Invalid rule format, try again.");
                (None, 1)
            }
        }
    }
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

pub fn await_input() -> (Option<(Position, usize)>, String) {
    loop {
        println!("Enter a command ((m)ove row column value, (s)ave filename, (q)uit, (f)inish with solver):");

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();

        let parts: Vec<&str> = input.split_whitespace().collect();
        if parts.is_empty() {
            println!("Invalid input. Please enter a command.");
            continue;
        }

        match parts[0] {
            "m" => {
                if parts.len() != 4 {
                    println!("Invalid input. Please enter: move row column value.");
                    continue;
                }

                let pos = match Position::new(
                    parts[1].parse().unwrap_or_default(),
                    parts[2].parse().unwrap_or_default(),
                ) {
                    Some(pos) => pos,
                    None => {
                        println!("Invalid row or column. Please enter valid numbers.");
                        continue;
                    }
                };

                let value: usize = match parts[3].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid value. Please enter a valid number.");
                        continue;
                    }
                };

                return (Some((pos, value)), String::from(""));
            }
            "s" => {
                if parts.len() != 2 {
                    println!("Invalid input. Please enter: save filename.");
                    continue;
                }

                return (None, String::from(parts[1]));
            }
            "f" => {
                return (None, String::from("finish"));
            }
            "q" => {
                println!("Quitting the game.");
                return (None, String::from("quit"));
            }
            _ => {
                println!("Invalid command. Please enter: move, save, or quit.");
            }
        }
    }
}
