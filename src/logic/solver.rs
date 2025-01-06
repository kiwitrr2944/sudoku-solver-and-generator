use super::board::Board;
use super::game::Game;
use super::rules::{Rule, RuleCheckResult};

pub struct Solver {
    pub board: Board,
    solution: Option<Board>,
    rules: Vec<Rule>,
}

impl Solver {
    pub fn new(game: Game) -> Self {
        Solver {
            board: game.board(),
            solution: None,
            rules: game.rules(),
        }
    }

    fn check_rules(&self) -> usize {
        let mut ret = (0, 0);

        for rule in &self.rules {
            let result = rule.check(&self.board);

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
        
        let pos = self.board.get_next_position();
        
        match pos {
            Some(pos) => {
                for value in 1..=self.board.get_side() {
                    self.board.set_value(pos, value);
        
                    if self.solve_recursive() {
                        return true;
                    }
        
                    self.board.clear_value(pos);
                }
                return false;
            }
            None => {
                if state == 2 {
                    self.solution = Some(self.board.clone());
                    return true;
                }
                else {
                    return false;    
                }
            }
        }
    }

    pub fn get_solution(&self) -> Option<Board> {
        self.solution.clone()
    }
}
