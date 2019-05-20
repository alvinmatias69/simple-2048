use std::io;

use super::interface::InputInterface;
use crate::input::Input;

pub struct InputString {}

impl InputInterface for InputString {
    fn get_user_input(&self) -> Input {
        let mut input = String::new();

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                if input == "up" {
                    return Input::Up;
                } else if input == "down" {
                    return Input::Down;
                } else if input == "left" {
                    return Input::Left;
                } else if input == "right" {
                    return Input::Right;
                }
            }
            Err(error) => println!("error: {}", error),
        }

        Input::Up
    }
}
