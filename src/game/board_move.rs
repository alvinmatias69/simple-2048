use crate::input::Input;

pub struct BoardMove {
    field: Vec<Vec<u32>>,
    direction: Input,
    width: usize,
    height: usize,
}

impl BoardMove {
    pub fn new(field: Vec<Vec<u32>>, direction: Input) -> BoardMove {
        let height = field.len();
        let width = field[0].len();
        let board_move = BoardMove {
            field,
            direction,
            height,
            width,
        };
        board_move
    }

    pub fn moved(&mut self) -> Vec<Vec<u32>> {
        let y_list: Vec<usize>;
        let x_list: Vec<usize>;

        match self.direction {
            Input::Up => {
                y_list = (0..self.width).collect();
                x_list = (0..self.height).collect();
            }
            Input::Down => {
                y_list = (0..self.width).collect();
                x_list = (0..self.height).rev().collect();
            }
            Input::Left => {
                y_list = (0..self.height).collect();
                x_list = (0..self.width).collect();
            }
            Input::Right => {
                y_list = (0..self.height).collect();
                x_list = (0..self.width).rev().collect();
            }
        }

        self.process_move(y_list, x_list);
        self.clone_field()
    }

    fn clone_field(&self) -> Vec<Vec<u32>> {
        let mut field: Vec<Vec<u32>> = Vec::with_capacity(self.height);
        for y in 0..self.height {
            let mut row: Vec<u32> = Vec::with_capacity(self.width);
            for x in 0..self.width {
                row.push(self.field[y][x]);
            }
            field.push(row);
        }
        field
    }

    fn process_move(&mut self, y_list: Vec<usize>, x_list: Vec<usize>) {
        for y in y_list.iter() {
            let mut added_list: Vec<bool> = vec![false; self.width + self.height];
            for x in x_list.iter() {
                let mut index: i32 = self.get_index(*x);
                let mut stop = false;
                let mut added = false;
                let mut value = self.get_current(*y, *x);
                let mut position = *x;

                while !stop && self.get_condition(index) {
                    let current = self.get_current(*y, index as usize);
                    if current == 0 {
                        position = index as usize;
                    } else if current == value && !added && !added_list[index as usize] {
                        added = true;
                        value *= 2;
                        position = index as usize;
                    } else {
                        stop = true;
                    }

                    index = self.get_next_index(index);
                }

                if position != *x && value != 0 {
                    added_list[position] = added;
                    self.set_value(*y, position, value);
                    self.set_value(*y, *x, 0);
                }
            }
        }
    }

    fn get_index(&self, x: usize) -> i32 {
        match self.direction {
            Input::Left => return x as i32 - 1,
            Input::Right => return x as i32 + 1,
            Input::Up => return x as i32 - 1,
            Input::Down => return x as i32 + 1,
        }
    }

    fn get_current(&self, y: usize, x: usize) -> u32 {
        match self.direction {
            Input::Left => return self.field[y][x],
            Input::Right => return self.field[y][x],
            Input::Up => return self.field[x][y],
            Input::Down => return self.field[x][y],
        }
    }

    fn get_condition(&self, index: i32) -> bool {
        match self.direction {
            Input::Left => return index >= 0,
            Input::Right => return index < self.width as i32,
            Input::Up => return index >= 0,
            Input::Down => return index < self.height as i32,
        }
    }

    fn get_next_index(&self, index: i32) -> i32 {
        match self.direction {
            Input::Left => return index - 1,
            Input::Right => return index + 1,
            Input::Up => return index - 1,
            Input::Down => return index + 1,
        }
    }

    fn set_value(&mut self, y: usize, x: usize, value: u32) {
        match self.direction {
            Input::Left => self.field[y][x] = value,
            Input::Right => self.field[y][x] = value,
            Input::Up => self.field[x][y] = value,
            Input::Down => self.field[x][y] = value,
        }
    }
}
