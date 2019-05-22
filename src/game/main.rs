// use crate::interface::input::interface::InputInterface;
// use crate::interface::output::interface::OutputInterface;

use super::board::{Board, BoardInterface};
use crate::input::Input;

use super::display::board;
use super::display::config;
use super::display::header;
use event::KeyCode;
use ggez::{conf, event, graphics, input, timer, Context, ContextBuilder, GameResult};
use input::keyboard;
use keyboard::KeyMods;

pub struct Game {
    board: Board,
    updated: bool,
    moved: bool,
    direction: Input,
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.moved {
            self.moved = false;
            let direction: Input;
            match self.direction {
                Input::Up => direction = Input::Up,
                Input::Down => direction = Input::Down,
                Input::Left => direction = Input::Left,
                Input::Right => direction = Input::Right,
            }
            self.updated = true;
            self.board.move_to(direction);
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::from_rgb(240, 236, 223));
        header::draw_title(ctx)?;
        header::draw_subtitle(ctx)?;
        header::draw_score(ctx, self.board.score)?;
        board::draw_board(ctx)?;
        board::draw_tiles(ctx, &self.board.field)?;

        if self.updated {
            self.updated = false;
        }

        graphics::present(ctx)?;
        timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        ctx: &mut Context,
        keycode: KeyCode,
        keymods: KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::Up | KeyCode::K | KeyCode::W => {
                self.moved = true;
                self.direction = Input::Up;
            }
            KeyCode::Down | KeyCode::J | KeyCode::S => {
                self.moved = true;
                self.direction = Input::Down;
            }
            KeyCode::Left | KeyCode::H | KeyCode::A => {
                self.moved = true;
                self.direction = Input::Left;
            }
            KeyCode::Right | KeyCode::L | KeyCode::D => {
                self.moved = true;
                self.direction = Input::Right;
            }
            KeyCode::Q => {
                if keymods.contains(KeyMods::CTRL) {
                    ggez::quit(ctx);
                }
            }
            _ => {}
        }
    }
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let board = BoardInterface::new(width, height);
        let game = Game {
            board,
            updated: false,
            moved: false,
            direction: Input::Up,
        };
        game
    }

    pub fn start(&mut self) {
        let cb = ContextBuilder::new("2048", "mat")
            .window_setup(conf::WindowSetup {
                title: "2048".to_owned(),
                samples: conf::NumSamples::Zero,
                vsync: true,
                transparent: false,
                icon: "".to_owned(),
                srgb: true,
            })
            .window_mode(conf::WindowMode {
                width: config::WINDOW_WIDTH,
                height: config::WINDOW_HEIGHT,
                maximized: false,
                fullscreen_type: conf::FullscreenType::Windowed,
                borderless: true,
                min_width: 0.0,
                max_width: 0.0,
                min_height: 0.0,
                max_height: 0.0,
                hidpi: false,
                resizable: false,
            });

        let (ref mut ctx, ref mut event_loop) = &mut cb.build().unwrap();
        event::run(ctx, event_loop, self).unwrap();
    }

    // Debug mode only
    //
    // pub fn start_debug(&mut self, input: impl InputInterface, output: impl OutputInterface) {
    //     while !self.board.is_finished() {
    //         output.show(&self.board.field);
    //         let direction = input.get_user_input();
    //         self.board.move_to(direction);
    //     }
    // }
}
