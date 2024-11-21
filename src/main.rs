use userinput::await_rule;

use crate::board::*;
use crate::rules::*;
use crate::solver::*;
use std::io::{self};

mod board;
mod rules;
mod solver;
mod userinput;

struct Game {
    board: Board,
    rules: Vec<Rule>,
}

impl Game {
    fn new(side: usize, sub_rows: usize, sub_cols: usize) -> Self {
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

    fn add_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }

    fn check_rules(&self) -> (Option<Vec<String>>, Option<Vec<String>>) {
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

        if violations.is_empty() {
            (None, Some(pending))
        } else {
            (Some(violations), Some(pending))
        }
    }
}

fn main() {
    let mut game = Game::new(4, 2, 2); // 3x3 Sudoku board

    loop {
        let x = await_rule();
        if x.1 == 0 {
            break;
        } else if x.1 == 2 {
            game.add_rule(x.0.unwrap());
        }
    }

    // loop {
    //     game.board.display();

    //     if !game.input_and_set_value() {
    //         break;
    //     }

    //     match game.check_rules() {
    //         (Some(violations), Some(pending)) => {
    //             println!("Violations:");
    //             for violation in violations {
    //                 println!("{}", violation);
    //             }

    //             println!("Pending:");
    //             for pending_rule in pending {
    //                 println!("{}", pending_rule);
    //             }
    //         }
    //         (Some(violations), None) => {
    //             println!("Violations:");
    //             for violation in violations {
    //                 println!("{}", violation);
    //             }
    //         }
    //         (None, Some(pending)) => {
    //             println!("Pending:");
    //             for pending_rule in pending {
    //                 println!("{}", pending_rule);
    //             }
    //         }
    //         (None, None) => {
    //             println!("All rules satisfied!");
    //         }
    //     }
    // }

    let mut s = Solver::new(game);
    s.solve();
    s.display_solutions();
}
