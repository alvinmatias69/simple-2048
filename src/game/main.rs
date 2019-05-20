use crate::interface::input::interface::InputInterface;
use crate::interface::output::interface::OutputInterface;

use super::board::{Board, BoardInterface};

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let board = BoardInterface::new(width, height);
        let game = Game { board };
        game
    }

    pub fn start(&mut self, input: impl InputInterface, output: impl OutputInterface) {
        while !self.board.is_finished() {
            output.show(&self.board.field);
            let direction = input.get_user_input();
            self.board.move_to(direction);
        }
    }
}
