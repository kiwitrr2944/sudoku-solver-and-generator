use super::board::{Board, Position};
use super::game::Game;
use super::rules::{Rule, RuleCheckResult};
use rand::seq::SliceRandom;
use crate::for_pos;

pub struct Solver {
    pub board: Board,
    solution: Option<Board>,
    rules: Vec<Rule>,
    n: usize,
    options: Vec<Vec<Vec<usize>>>,
    position_rules: Vec<Vec<Vec<usize>>>,
    ok: bool,
    rng: rand::rngs::ThreadRng,
    random: bool,
}

impl Solver {
    pub fn new(game: Game, random: bool) -> Self {
        let n = game.get_side();
        let options = vec![vec![(1..=n).collect(); n]; n];
        let mut position_rules = vec![vec![vec![]; n]; n];

        for rule in &game.rules() {
            dbg!(rule);
            for pos in rule.get_positions() {
                let row = pos.row() - 1;
                let col = pos.col() - 1;
                position_rules[row][col].push(rule.get_index());
            }
        }

        let mut ret = Solver {
            board: game.board(),
            solution: None,
            rules: game.rules(),
            n,
            options,
            position_rules,
            ok: true,
            rng: rand::thread_rng(),
            random,
        };

        let board = game.board();

        for_pos!(n, |pos| {
            let v = board.get_value(pos);
            if board.get_value(pos) > 0 {
                let ok = ret.place(pos, v);
                ret.ok &= ok;
            }
        });

        dbg!(ret.ok);
        ret
    }

    fn place(&mut self, pos: Position, digit: usize) -> bool {
        let row = pos.row() - 1;
        let col = pos.col() - 1;

        // Save the current state to restore later if needed
        let original_value = self.board.get_value(pos);
        let original_options = self.options.clone();

        if !self.options[row][col].contains(&digit) {
            return false;
        }
        // Place the digit
        self.board.set_value(pos, digit);

        // Check if placing the digit violates any rules
        for &rule_index in &self.position_rules[row][col] {
            let rule = &self.rules[rule_index];
            if let RuleCheckResult::Critical(_) = rule.check(&self.board) {
                self.board.set_value(pos, original_value);
                self.options = original_options;
                return false;
            }
        }

        // Propagate possible values
        for &rule_index in &self.position_rules[row][col] {
            let rule = &self.rules[rule_index];
            for pos in rule.get_positions() {
                let r = pos.row() - 1;
                let c = pos.col() - 1;
                if self.board.get_value(pos) == 0 {
                    self.options[r][c].retain(|&x| x != digit);
                    if self.options[r][c].is_empty() {
                        // Restore the original state
                        self.board.set_value(pos, original_value);
                        self.options = original_options;
                        return false;
                    }
                }
            }
        }

        true
    }

    fn unplace(&mut self, pos: Position) {
        let row = pos.row() - 1;
        let col = pos.col() - 1;

        let original_value = self.board.get_value(pos);
        if original_value == 0 {
            return;
        }
        // Restore the original value
        self.board.set_value(pos, 0);

        // Recalculate options for affected positions
        for &rule_index in &self.position_rules[row][col] {
            let rule = &self.rules[rule_index];
            if let Rule::Permutation(_) = rule {
                for pos in rule.get_positions() {
                    let r = pos.row() - 1;
                    let c = pos.col() - 1;
                    if self.board.get_value(pos) == 0 {
                        self.options[r][c].push(original_value);
                    }
                }
            }
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
        if !self.ok {
            self.solution = None;
            return;
        }

        self.solve_recursive();
    }

    pub fn get_options(&self, pos: Position) -> Vec<usize> {
        self.options[pos.row() - 1][pos.col() - 1].clone()
    }

    fn get_next_position(&self) -> Option<Position> {
        let mut ret: (Vec<usize>, Option<Position>) = (vec![], None);

        for_pos!(self.n, |pos| {
            if self.board.get_value(pos) == 0 {
                let options = &self.get_options(pos);
                if (!options.is_empty()) && (ret.1.is_none() || options.len() < ret.0.len()) {
                    ret = (options.clone(), Some(pos));
                }
            }
        });

        ret.1
    }

    pub fn solve_recursive(&mut self) -> bool {
        let pos = self.get_next_position();

        dbg!(pos);
        print!("{}", self.board);

        match pos {
            Some(pos) => {
                let mut opt = self.options[pos.row() - 1][pos.col() - 1].clone();
                if self.random {
                    opt.shuffle(&mut self.rng);
                }
                for value in opt {
                    let ok = self.place(pos, value);
                    if ok && self.solve_recursive() {
                        return true;
                    }
                    self.unplace(pos);
                }
                false
            }
            None => {
                let state = self.check_rules();
                if state == 2 {
                    self.solution = Some(self.board.clone());
                    true
                } else {
                    false
                }
            }
        }
    }

    pub fn get_solution(&self) -> Option<Board> {
        self.solution.clone()
    }

    pub fn generate(mut game: Game) -> Game {
        let mut solver = Solver::new(game.clone(), true);
        solver.random = true;
        solver.solve();
        let mut x = solver.get_solution().unwrap();

        let mut positions: Vec<Position> = (1..=solver.n)
            .flat_map(|row| (1..=solver.n).filter_map(move |col| Position::new(row, col)))
            .collect();
        positions.shuffle(&mut solver.rng);

        for pos in positions.into_iter().take(solver.n * solver.n / 3 * 2) {
            x.set_value(pos, 0);
        }

        game.set_board(x);
        game
    }
}
