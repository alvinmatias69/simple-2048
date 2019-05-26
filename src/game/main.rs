// use crate::interface::input::interface::InputInterface;
// use crate::interface::output::interface::OutputInterface;

use super::board::{Board, BoardInterface};
use crate::input::Input;

use super::display::{board, config, header, menu};
use super::sound;
use sound::SoundType;

use event::KeyCode;
use ggez::{conf, event, graphics, input, timer, Context, ContextBuilder, GameResult};
use input::{keyboard, mouse};
use keyboard::KeyMods;
use mouse::MouseButton;

pub struct Game {
    board: Board,
    updated: bool,
    moved: bool,
    direction: Input,
    width: u32,
    height: u32,
    win: bool,
    lose: bool,
    continue_game: bool,
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.moved {
            self.moved = false;
            let direction: Input;
            match self.direction {
                Input::Up => direction = Input::Up,
                Input::Down => direction = Input::Down,
                Input::Left => direction = Input::Left,
                Input::Right => direction = Input::Right,
            }
            self.updated = self.board.move_to(direction);
            self.lose = self.board.is_finished();
            self.win = self.board.highest_tile > 1024;

            if self.win && !self.continue_game {
                sound::play(ctx, SoundType::Win)?;
            } else if self.lose {
                sound::play(ctx, SoundType::Lose)?;
            } else if self.updated {
                sound::play(ctx, SoundType::Move)?;
            } else {
                sound::play(ctx, SoundType::UnableToMove)?;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        if self.updated {
            self.updated = false;
            graphics::clear(ctx, graphics::Color::from_rgb(240, 236, 223));
            header::draw_title(ctx)?;
            header::draw_subtitle(ctx)?;
            header::draw_score(ctx, self.board.score)?;
            board::draw_board(ctx)?;
            board::draw_tiles(ctx, &self.board.field)?;
            if self.win && !self.continue_game {
                menu::draw(ctx, true)?;
            }
            if self.lose {
                menu::draw(ctx, false)?;
            }
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
                if !(self.win && !self.continue_game) {
                    self.moved = true;
                    self.direction = Input::Up;
                }
            }
            KeyCode::Down | KeyCode::J | KeyCode::S => {
                if !(self.win && !self.continue_game) {
                    self.moved = true;
                    self.direction = Input::Down;
                }
            }
            KeyCode::Left | KeyCode::H | KeyCode::A => {
                if !(self.win && !self.continue_game) {
                    self.moved = true;
                    self.direction = Input::Left;
                }
            }
            KeyCode::Right | KeyCode::L | KeyCode::D => {
                if !(self.win && !self.continue_game) {
                    self.moved = true;
                    self.direction = Input::Right;
                }
            }
            KeyCode::Return => {
                if self.win && !self.continue_game {
                    self.continue_game = true;
                    self.updated = true;
                }
            }
            KeyCode::Q => {
                if keymods.contains(KeyMods::CTRL) {
                    ggez::quit(ctx);
                }
            }
            KeyCode::N => {
                if keymods.contains(KeyMods::CTRL) {
                    self.reset(ctx);
                }
            }
            _ => {}
        }
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        if x >= config::MENU_BUTTON_POSITION.0
            && x <= config::MENU_BUTTON_POSITION.0 + config::MENU_BUTTON_SIZE.0
            && y >= config::MENU_BUTTON_POSITION.1
            && y <= config::MENU_BUTTON_POSITION.1 + config::MENU_BUTTON_SIZE.1
        {
            if self.win && !self.continue_game {
                self.continue_game = true;
                self.updated = true;
                mouse::set_cursor_hidden(ctx, true);
                sound::play(ctx, SoundType::Menu);
            } else if self.lose {
                self.reset(ctx);
                self.updated = true;
                mouse::set_cursor_hidden(ctx, true);
                sound::play(ctx, SoundType::Menu);
            }
        }
    }
}

impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        let board = BoardInterface::new(width, height);
        let game = Game {
            board,
            updated: true,
            moved: false,
            direction: Input::Up,
            width,
            height,
            win: false,
            lose: false,
            continue_game: false,
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
                icon: "/2048.ico".to_owned(),
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
        sound::play(ctx, SoundType::Start);
        mouse::set_cursor_hidden(ctx, true);
        event::run(ctx, event_loop, self).unwrap();
    }

    fn reset(&mut self, ctx: &mut Context) {
        let board: Board = BoardInterface::new(self.width, self.height);
        self.board = board;
        self.win = false;
        self.lose = false;
        self.continue_game = false;
        self.updated = true;
        sound::play(ctx, SoundType::Start);
        mouse::set_cursor_hidden(ctx, true);
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
