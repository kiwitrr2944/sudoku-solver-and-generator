use serde::{Deserialize, Serialize};
use std::fmt::{self, Display};

const N: usize = 9;

#[derive(Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Option<Self> {
        if (1..=N).contains(&row) && (1..=N).contains(&col) {
            Some(Position { row, col })
        } else {
            None
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        let row = index % N + 1;
        let col = index / N + 1;
        Position::new(row, col)
    }

    pub fn coords(&self) -> (usize, usize) {
        (self.row - 1, self.col - 1)
    }

    pub fn index(&self) -> usize {
        (self.row - 1) + (self.col - 1) * N
    }

    pub fn default_color(&self, r: usize, c: usize) -> usize {
        7 + (((self.row - 1) / r + (self.col - 1) / c) % 2)
    }

    pub fn row(&self) -> usize {
        self.row
    }

    pub fn col(&self) -> usize {
        self.col
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
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

    pub fn get_value(&self, pos: Position) -> usize {
        self.board[pos.row - 1][pos.col - 1]
    }

    pub fn get_side(&self) -> usize {
        self.side
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
