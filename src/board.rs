use serde::{Deserialize, Serialize};

const SIDE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Option<Self> {
        if (1..=SIDE).contains(&row) && (1..=SIDE).contains(&col) {
            Some(Position { row, col })
        } else {
            None
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    side: usize,
    size: usize,
    filled: usize,
    board: Vec<Vec<Option<usize>>>,
}

impl Board {
    pub fn new() -> Self {
        let size = SIDE * SIDE;

        Board {
            side: SIDE,
            size,
            filled: 0,
            board: vec![vec![None; SIDE]; SIDE],
        }
    }

    pub fn display(&self) {
        for row in &self.board {
            for &cell in row {
                print!(
                    "{} ",
                    match cell {
                        Some(x) => x.to_string(),
                        None => String::from("_"),
                    }
                );
            }
            println!();
        }
        println!();
    }

    pub fn set_value(&mut self, pos: Position, value: usize) {
        if self.board[pos.row - 1][pos.col - 1].is_none() {
            self.filled += 1;
            self.board[pos.row - 1][pos.col - 1] = Some(value);
        }
    }

    pub fn get_value(&self, pos: Position) -> Option<usize> {
        self.board[pos.row - 1][pos.col - 1]
    }

    pub fn get_side(&self) -> usize {
        self.side
    }

    pub fn is_filled(&self) -> bool {
        self.filled == self.size
    }

    pub fn get_next_position(&self) -> Option<Position> {
        for row in 1..=SIDE {
            for col in 1..=SIDE {
                let pos = Position::new(row, col).unwrap();
                if self.get_value(pos).is_none() {
                    return Some(pos);
                }
            }
        }
        None
    }

    pub fn clear_value(&mut self, pos: Position) {
        if self.board[pos.row - 1][pos.col - 1].is_some() {
            self.filled -= 1;
        }
        self.board[pos.row - 1][pos.col - 1] = None;
    }
}
