use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

#[warn(unknown_lints, reason="CHANGEDIMENSION")]
const SIDE: usize = 6; 
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

    pub fn index(&self) -> usize {
        (self.row - 1) + (self.col - 1)*SIDE
    }

    pub fn get_sub_id(&self, r: usize, c: usize) -> usize {
        (self.row - 1) / r + (self.col - 1) / c
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Board {
    side: usize,
    filled: usize,
    board: Vec<Vec<usize>>,
}

impl Board {
    pub fn new(side: usize) -> Self {
        Board {
            side,
            filled: 0,
            board: vec![vec![0; side]; side],
        }
    }

    pub fn set_value(&mut self, pos: Position, value: usize) {
        self.board[pos.row - 1][pos.col - 1] = value;
    }

    pub fn get_value(&self, pos: Option<Position>) -> usize {
        match pos {
            Some(pos) => self.board[pos.row - 1][pos.col - 1],
            None => 0,
        }
    }

    pub fn get_side(&self) -> usize {
        self.side
    }

    pub fn clear_value(&mut self, pos: Position) {
        self.set_value(pos, 0);
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            for &cell in row {
                let cell_str = if cell == 0 { "_" } else { &cell.to_string() };
                write!(f, "{} ", cell_str)?;
            }
            writeln!(f)?;
        }
        writeln!(f)
    }
}