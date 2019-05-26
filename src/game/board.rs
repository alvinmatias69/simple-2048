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
        self.empty.is_empty() && !self.find_pair()
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

    fn find_pair(&self) -> bool {
        let mut y: usize = 0;
        let mut found = false;

        while !found && y < self.height {
            let mut x: usize = 0;
            while !found && x < self.width {
                found = self.check_pair(x, y);
                x += 1;
            }
            y += 1;
        }
        found
    }

    fn check_pair(&self, x: usize, y: usize) -> bool {
        let value = self.field[y][x];
        let combination: [i32; 3] = [-1, 0, 1];
        let mut found: bool = false;

        let mut y_idx: usize = 0;
        while !found && y_idx < 3 {
            let mut x_idx: usize = 0;
            let cur_y = y as i32 + combination[y_idx];
            if cur_y >= 0 && cur_y < self.height as i32 {
                while !found && x_idx < 3 {
                    let cur_x = x as i32 + combination[x_idx];
                    if cur_x >= 0
                        && cur_x < self.width as i32
                        && !(cur_x == x as i32 && cur_y == y as i32)
                    {
                        found = value == self.field[cur_y as usize][cur_x as usize];
                    }
                    x_idx += 1;
                }
            }
            y_idx += 1;
        }

        found
    }
}
