use super::board_move::BoardMove;
use super::random;
use crate::input::Input;

pub struct Board {
    pub field: Vec<Vec<u32>>,
    pub score: u32,
    width: usize,
    height: usize,
    empty: Vec<(usize, usize)>,
    pub highest_tile: u32,
}

pub trait BoardInterface {
    fn new(width: u32, height: u32) -> Self;
    fn is_finished(&self) -> bool;
    fn move_to(&mut self, direction: Input) -> bool;
}

impl BoardInterface for Board {
    fn new(width: u32, height: u32) -> Self {
        let column = vec![0; width as usize];
        let field = vec![column; height as usize];

        let mut empty: Vec<(usize, usize)> = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                empty.push((y as usize, x as usize));
            }
        }

        let mut board = Board {
            field,
            score: 0,
            empty,
            width: width as usize,
            height: height as usize,
            highest_tile: 0,
        };

        board.spawn_random_number();
        board
    }

    fn is_finished(&self) -> bool {
        self.empty.is_empty()
    }

    fn move_to(&mut self, direction: Input) -> bool {
        let mut field: Vec<Vec<u32>> = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row: Vec<u32> = Vec::with_capacity(self.width);
            for x in 0..self.width {
                row.push(self.field[y][x]);
            }
            field.push(row);
        }

        let mut board_move = BoardMove::new(field, direction);
        match board_move.moved() {
            Ok(field) => {
                self.field = field;
                self.score += board_move.score;
                self.update_empty();
                self.spawn_random_number();
                if board_move.highest_tile > self.highest_tile {
                    self.highest_tile = board_move.highest_tile;
                }
                true
            }
            Err(_) => false,
        }
    }
}

impl Board {
    fn spawn_random_number(&mut self) {
        let empty: usize;
        if self.empty.len() > 2 {
            empty = 2;
        } else {
            empty = self.empty.len();
        }

        for _ in 0..empty {
            let index = random::between(0, self.empty.len() as u32);
            let (y, x) = self.empty[index as usize];
            self.field[y][x] = 2;
            self.empty.remove(index as usize);
        }
    }

    fn update_empty(&mut self) {
        let mut empty: Vec<(usize, usize)> = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if self.field[y][x] == 0 {
                    empty.push((y, x));
                }
            }
        }
        self.empty = empty;
    }
}
