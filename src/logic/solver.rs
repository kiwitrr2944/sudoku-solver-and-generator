use super::board::{Board, Position};
use super::game::Game;
use super::rules::{Rule, RuleCheckResult};
use crate::for_pos;
use rand::seq::SliceRandom;

pub struct Solver {
    board: Board,
    solution: Option<Board>,
    rules: Vec<Rule>,
    n: usize,
    options: Vec<Vec<Vec<usize>>>,
    position_rules: Vec<Vec<Vec<usize>>>,
    is_valid: bool,
    rng: rand::rngs::ThreadRng,
    use_randomization: bool,
}

impl Solver {
    pub fn new(game: Game, use_randomization: bool) -> Self {
        let n = game.get_side();

        let mut options: Vec<Vec<Vec<usize>>> = vec![vec![(1..=n).collect(); n]; n];
        let mut position_rules: Vec<Vec<Vec<usize>>> = vec![vec![vec![]; n]; n];

        let rules: Vec<Rule> = game
            .rules()
            .into_iter()
            .filter(|rule| match rule {
                Rule::Permutation(_r) => rule.get_positions().len() == n,
                Rule::Relation(_r) => rule.get_positions().len() == 2,
                _ => true,
            })
            .collect();

        rules.iter().for_each(|rule| {
            if let Rule::Permutation(_r) = rule {
                for pos in rule.get_positions() {
                    let row = pos.row() - 1;
                    let col = pos.col() - 1;
                    position_rules[row][col].push(rule.get_index());
                }
            } else if let Rule::Relation(_r) = rule {
                let pos1 = rule.get_positions()[0];
                let pos2 = rule.get_positions()[1];
                if options[pos1.row() - 1][pos1.col() - 1].last().unwrap_or(&0) == &(n - 1) {
                    options[pos1.row() - 1][pos1.col() - 1].pop();
                }
                if options[pos2.row() - 1][pos2.col() - 1]
                    .first()
                    .unwrap_or(&0)
                    == &0
                {
                    options[pos2.row() - 1][pos2.col() - 1].remove(0);
                }
            }
        });

        let mut ret = Solver {
            board: game.board(),
            solution: None,
            rules,
            n,
            options,
            position_rules,
            is_valid: true,
            rng: rand::thread_rng(),
            use_randomization,
        };

        let board = game.board();

        for_pos!(n, |pos| {
            let v = board.get_value(pos);
            if board.get_value(pos) > 0 {
                let ok = ret.place(pos, v);
                ret.is_valid &= ok;
            }
        });

        ret
    }

    fn place(&mut self, pos: Position, digit: usize) -> bool {
        let (row, col) = pos.coords();

        let original_value = self.board.get_value(pos);
        let original_options = self.options.clone();

        if !self.options[row][col].contains(&digit) {
            return false;
        }

        self.board.set_value(pos, digit);

        for &rule_index in &self.position_rules[row][col] {
            let rule = &self.rules[rule_index];
            if let RuleCheckResult::Critical(_) = rule.check(&self.board) {
                self.board.set_value(pos, original_value);
                self.options = original_options;
                return false;
            }
        }

        for &rule_index in &self.position_rules[row][col] {
            let rule = &self.rules[rule_index];
            if let Rule::Permutation(_) = rule {
                for pos in rule.get_positions() {
                    let (r, c) = pos.coords();

                    if self.board.get_value(pos) == 0 {
                        self.options[r][c].retain(|&x| x != digit);
                        if self.options[r][c].is_empty() {
                            self.board.set_value(pos, original_value);
                            self.options = original_options;
                            return false;
                        }
                    }
                }
            }
        }

        true
    }

    fn unplace(&mut self, pos: Position) {
        let (row, col) = pos.coords();

        let original_value = self.board.get_value(pos);
        if original_value == 0 {
            return;
        }

        self.board.set_value(pos, 0);

        for &rule_index in &self.position_rules[row][col] {
            let rule = &self.rules[rule_index];
            if let Rule::Permutation(_) = rule {
                for pos in rule.get_positions() {
                    let (r, c) = pos.coords();
                    if self.board.get_value(pos) == 0
                        && !self.options[r][c].contains(&original_value)
                    {
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
        if !self.is_valid {
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

        match pos {
            Some(pos) => {
                let mut opt = self.get_options(pos);
                if self.use_randomization {
                    opt.shuffle(&mut self.rng);
                }
                for value in opt {
                    if self.place(pos, value) && self.solve_recursive() {
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

    pub fn generate(mut game: Game) -> Option<Game> {
        let mut solver = Solver::new(game.clone(), true);

        solver.solve();
        solver.get_solution()?;

        let mut part_board = solver.get_solution().unwrap();

        let mut positions: Vec<Position> = (1..=solver.n)
            .flat_map(|row| (1..=solver.n).filter_map(move |col| Position::new(row, col)))
            .collect();

        positions.shuffle(&mut solver.rng);

        for pos in positions.into_iter().take(solver.n * solver.n / 3 * 2) {
            part_board.set_value(pos, 0);
        }

        game.set_board(part_board);
        Some(game)
    }
}
