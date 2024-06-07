use crate::{snake::Snake, utils::ScreenConstruct};

use crossterm::{
    //cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::{error::Error, io::Stdout, time::Duration};

pub const SNAKE_ICON: &str = "\x1b[1;31m▓\x1b[0m";

pub enum IsQuit {
    Quit,
    DontQuit,
}

pub enum Direction {
    North, // w, up
    East,  // d, right
    South, // s, down
    West,  // a, left
    NoDirection,
}

impl Direction {
    pub fn is_direction(code: crossterm::event::KeyCode) -> Direction {
        match code {
            KeyCode::Char('w') | KeyCode::Up => return Direction::North,
            KeyCode::Char('a') | KeyCode::Left => return Direction::West,
            KeyCode::Char('s') | KeyCode::Down => return Direction::South,
            KeyCode::Char('d') | KeyCode::Right => return Direction::East,
            _ => return Direction::NoDirection,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Game<'a> {
    screen: ScreenConstruct<'a>,
    snake: Snake,
}

impl<'a> Game<'a> {
    pub fn instantiate_game(screen: ScreenConstruct<'a>, snake: Snake) -> Self {
        Self { screen, snake }
    }
    pub fn listener(&mut self, stdout: &mut Stdout) -> Result<IsQuit, Box<dyn Error>> {
        loop {
            if event::poll(Duration::from_millis(33)).expect("Failed to poll for events") {
                // event reader, attempts to read event every 33ms (30fps)
                match event::read()? {
                    Event::Key(event) => match event.code {
                        KeyCode::Char('c') => {
                            if event.modifiers.contains(KeyModifiers::CONTROL) {
                                return Ok(IsQuit::Quit);
                            } else {
                                return Ok(IsQuit::DontQuit);
                            }
                        } // ctrl c my beloved
                        _ => {}
                    },
                    _ => {}
                }
            }
            // after event/if no event
            self.render(stdout, None);
        }
    }
    pub fn render(&mut self, stdout: &mut Stdout, direction: Option<Direction>) {
        let _ = stdout.queue(Clear(ClearType::All));
        let _ = self.screen.update_screen_size();
        let _ = self.screen.create_border(stdout);
        match direction {
            Some(d) => {
                self.snake.update_snake(d, (self.screen.w, self.screen.h)) // i cant remember what i was doing i fucked myself
            }
            None => self
                .snake
                .update_snake(Direction::NoDirection, (self.screen.w, self.screen.h)), // fuck, i thought this fucntion needed to own self...
                                                                                       // i think that means something is fucked up
        }
    }
    pub fn draw_shit(&mut self, mut stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {}
}
