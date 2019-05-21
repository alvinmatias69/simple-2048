extern crate rand;

mod game;
mod input;
mod interface;

// Debug mode only
//
// use self::input_interface::InputInterface;
// use self::input_string::InputString;
// use self::interface::input::{interface as input_interface, string as input_string};

// use self::interface::output::{interface as output_interface, string as output_string};
// use self::output_interface::OutputInterface;
// use self::output_string::OutputString;

use self::game::main::Game;

fn main() {
    let mut game = Game::new(4, 4);
    game.start();

    // Debug mode only
    //
    // let input = InputString {};
    // let output = OutputString {};

    // game.start_debug(input, output);
}
