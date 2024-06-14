use crate::{snake::Snake, utils::ScreenConstruct};

use crossterm::{
    cursor::MoveTo,
    //cursor::MoveTo,
    event::{self, Event, KeyCode, KeyModifiers},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use rand::Rng;
use std::{
    error::Error,
    io::{Stdout, Write},
    process::exit,
    time::Duration,
};

// pub const SNAKE_ICON: &str = "\x1b[1;31m▓\x1b[0m";
pub const SNAKE_STEP: u16 = 2;
pub const FOOD: &str = "\x1b[1;35m▓\x1b[0m";

pub enum IsQuit {
    Quit,
    DontQuit,
}

#[derive(Clone, PartialEq, Debug)]
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
pub struct Food {
    xy: (u16, u16),
    is_hungry: bool,
}

impl Food {
    // returns new empty food
    pub fn new() -> Self {
        let is_hungry = true;
        Self {
            xy: (0, 0),
            is_hungry,
        }
    }
    pub fn generate_random_food() -> Result<Self, Box<dyn std::error::Error>> {
        let (w, h) = crossterm::terminal::size()?; // i probably don't need to make this call here
        let mut rng = rand::thread_rng();
        let random_x = rng.gen_range(2..w - 2);
        let random_y = rng.gen_range(2..h - 2);
        Ok(Self {
            xy: (random_x, random_y),
            is_hungry: true,
        })
    }
    pub fn draw_and_regenerate_food(
        &mut self,
        stdout: &mut Stdout,
        character: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if self.is_hungry == true {
            let _ = stdout.queue(MoveTo(self.xy.0, self.xy.1));
            let _ = stdout.write(character.as_bytes());
            let _ = stdout.flush().unwrap();

            let (w, h) = crossterm::terminal::size().expect("Terminal failed to get size");
            let mut rng = rand::thread_rng();
            let random_x = rng.gen_range(0..w - 1);
            let random_y = rng.gen_range(0..h - 1);
            self.xy = (random_x, random_y);
            self.is_hungry = false;
        } else {
            let _ = stdout.queue(MoveTo(self.xy.0, self.xy.1));
            let _ = stdout.write(character.as_bytes());
            let _ = stdout.flush().unwrap();
        }
        Ok(())
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Game<'a> {
    screen: ScreenConstruct<'a>,
    snake: Snake,
    food: Food,
    direction: Direction,
}

impl<'a> Game<'a> {
    pub fn instantiate_game(screen: ScreenConstruct<'a>, snake: Snake, food: Food) -> Self {
        Self {
            screen,
            snake,
            food,
            direction: Direction::NoDirection,
        } // TODO: Fix this ??
    }
    pub fn listener(&mut self, stdout: &mut Stdout) -> Result<IsQuit, Box<dyn Error>> {
        loop {
            if event::poll(Duration::from_millis(232)).expect("Failed to poll for events") {
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
                        _ => {
                            let direction = Direction::is_direction(event.code);
                            if direction != Direction::NoDirection {
                                self.direction = direction.clone()
                            }
                            self.render(stdout, Some(direction));
                        }
                    },
                    _ => {}
                }
            }
            // after event/if no event
            self.render(stdout, Some(self.direction.clone()));
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
                .update_snake(self.direction.clone(), (self.screen.w, self.screen.h)), // fuck, i thought this fucntion needed to own self...
                                                                                       // i think that means something is fucked up
        } // Direction::NoDirection is a skill issue cope solution.
        self.snake.draw_snake().unwrap();
        self.food
            .draw_and_regenerate_food(stdout, &FOOD.to_string())
            .expect("code has failed, must commit sepuku");
        self.is_on_food().unwrap();
        self.is_on_self().unwrap();
    }
    // pub fn draw_shit(&mut self, mut stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {}
    pub fn is_on_food(&mut self) -> Result<(), Box<dyn Error>> {
        if self.snake.head.position.xy == self.food.xy {
            self.snake.eat_food(self.food.xy).unwrap();
            self.food.is_hungry = true;
        }
        Ok(())
    }
    pub fn is_on_self(&mut self) -> Result<(), Box<dyn Error>> {
        for part in self.snake.body.iter() {
            if self.snake.head.position.xy == part.position.last_xy {
                self.snake.alive = false;
                let _ = terminal::disable_raw_mode();
                std::process::exit(0)
            } else {
            }
        }
        Ok(())
    }
}
