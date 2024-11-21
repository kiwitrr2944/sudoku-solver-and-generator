const SIDE: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    row: usize,
    col: usize,
}

impl Position {
    pub fn new(row: usize, col: usize) -> Option<Self> {
        if (1 <= row && row <= SIDE) && (1 <= col && col <= SIDE) {
            Some(Position { row, col })
        } else {
            None
        }
    }
}

#[derive(Clone)]
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
    }

    pub fn set_value(&mut self, pos: Position, value: usize) {
        if self.board[pos.row - 1][pos.col - 1].is_none() {
            self.filled += 1;
        }
        self.board[pos.row - 1][pos.col - 1] = Some(value);
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

    pub fn get_size(&self) -> usize {
        self.size
    }

    pub fn get_next_position(&self) -> Position {
        for row in 1..=self.side {
            for col in 1..=self.side {
                if self.board[row - 1][col - 1].is_none() {
                    return Position::new(row, col).unwrap();
                }
            }
        }
        Position::new(0, 0).unwrap()
    }

    pub fn clear_value(&mut self, pos: Position) {
        if self.board[pos.row - 1][pos.col - 1].is_some() {
            self.filled -= 1;
        }
        self.board[pos.row - 1][pos.col - 1] = None;
    }
}
