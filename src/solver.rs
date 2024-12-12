use crate::board::Board;
use crate::game::Game;
use crate::rules::{Rule, RuleCheckResult};

pub struct Solver {
    board: Board,
    solutions: Vec<Board>,
    rules: Vec<Rule>,
}

impl Solver {
    pub fn new(game: Game) -> Self {
        Solver {
            board: game.board(),
            solutions: Vec::new(),
            rules: game.rules(),
        }
    }

    fn check_rules(&self) -> usize {
        let mut ret = (0, 0);

        for rule in &self.rules {
            let result = match rule {
                Rule::Sum(r) => r.check(&self.board),
                Rule::Permutation(r) => r.check(&self.board),
                Rule::Relation(r) => r.check(&self.board),
            };

            match result {
                RuleCheckResult::Critical(_) => ret.0 += 1,
                RuleCheckResult::Unfulfilled(_) => ret.1 += 1,
                RuleCheckResult::Ok => continue,
            };
        }

        if ret.0 > 0 {
            0
        } else if ret.1 > 0 {
            1
        } else {
            2
        }
    }

    pub fn solve(&mut self) {
        self.solve_recursive();
    }

    pub fn solve_recursive(&mut self) {
        let state = self.check_rules();
        if self.board.is_filled() {
            if state == 2 {
                self.solutions.push(self.board.clone());
            }
            return;
        }

        if state == 0 {
            return;
        }

        let pos = self.board.get_next_position().unwrap();
        dbg!("solve", pos);
        for value in 1..=self.board.get_side() {
            self.board.set_value(pos, value);
            self.solve_recursive();
            self.board.clear_value(pos);
        }
    }

    pub fn display_solutions(self) {
        if self.solutions.is_empty() {
            println!("No solutions found");
            return;
        }
        for (i, solution) in self.solutions.iter().enumerate() {
            println!("Solution {}", i + 1);
            solution.display();
        }
    }
}
