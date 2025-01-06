use super::board::{Board, Position};
use super::game::Game;
use super::rules::{Rule, RuleCheckResult};

pub struct Solver {
    pub board: Board,
    solution: Option<Board>,
    rules: Vec<Rule>,
    current_pos: Option<Position>,
}

impl Solver {
    pub fn new(game: Game) -> Self {
        Solver {
            board: game.board(),
            solution: None,
            rules: game.rules(),
            current_pos: Position::new(1, 1),
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
        dbg!("solve");
        self.solve_recursive();
    }

    pub fn solve_recursive(&mut self) -> bool {
        let state = self.check_rules();
        if state == 0 {
            return false;
        }
        
        loop {
            match self.current_pos {
                Some(pos) => {
                    if (self.board.get_value(Some(pos))).is_some() {
                        self.current_pos = pos.next();
                    }
                    else {
                        break;
                    }
                }
                None => {
                    if state == 2 {
                        self.solution = Some(self.board.clone());
                        return true;
                    }
                }
            }
        }
        
        let pos = self.current_pos.unwrap();
        
        for value in 1..=self.board.get_side() {
            self.board.set_value(pos, value);

            if self.solve_recursive() {
                return true;
            }

            self.board.clear_value(pos);
        }
        false
    }

    pub fn get_solution(&self) -> Option<Board> {
        self.solution.clone()
    }

    // pub fn display_solutions(&self) {
    //     if self.solutions.is_empty() {
    //         println!("No solutions found");
    //         return;
    //     }
    //     for (i, solution) in self.solutions.iter().enumerate() {
    //         println!("Solution {}", i + 1);
    //         solution.display();
    //     }
    // }
}
