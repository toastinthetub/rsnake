use crate::{draw_cell_arbitrary, game::Direction};
// use crossterm::{
//     cursor::MoveTo,
//     style::{Color, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
//     terminal::{self, Clear, ClearType},
//     QueueableCommand,
// };
// use std::{
//     io::{Stdout, Write},
//     sync::Arc,
// };

#[derive(Clone, PartialEq, Debug)]
pub struct Position {
    pub xy: (u16, u16),
    pub last_xy: (u16, u16),
}

#[derive(Clone, PartialEq, Debug)]
pub struct SnakeHead {
    pub position: Position,
}

#[derive(Clone, PartialEq, Debug)]
pub struct SnakeBlock {
    pub position: Position,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Snake {
    pub head: SnakeHead,
    pub body: Vec<SnakeBlock>,
    pub alive: bool,
    pub length: i32,
}

impl Position {
    // empty position
    pub fn new() -> Self {
        Self {
            xy: (0, 0),
            last_xy: (0, 0),
        }
    }
}

impl SnakeHead {
    // empty snake head at top left cell
    pub fn new() -> Self {
        Self {
            position: Position::new(),
        }
    }
    pub fn draw(&mut self) {
        let mut stdout = std::io::stdout();
        draw_cell_arbitrary(
            &mut stdout,
            crate::utils::AnsiiColor::Blue,
            self.position.xy,
        )
    }
}

impl SnakeBlock {
    //empty snake block at top left cell
    pub fn new() -> Self {
        Self {
            position: Position::new(),
        }
    }
    pub fn draw(&mut self) {
        let mut stdout = std::io::stdout();
        draw_cell_arbitrary(
            &mut stdout,
            crate::utils::AnsiiColor::White,
            self.position.xy,
        )
    }
}

impl Snake {
    // empty snake consisting of empty snake anatomy Lolz
    pub fn new() -> Self {
        Self {
            head: SnakeHead::new(),
            body: Vec::new(),
            alive: true,
            length: 0,
        }
    }
    pub fn update_snake(
        &mut self,
        /* stdout: &mut Stdout, */ direction: Direction,
        size: (u16, u16),
    ) {
        // might have to borrow self
        // this is bad i know passing a field of an object on the same level but bear with me
        if self.head.position.xy.0 >= size.0 // if head has hit rightmost wall
            || self.head.position.xy.0 <= 0 // if head has hit leftmost wall
            || self.head.position.xy.1 <= size.1 // if head has hit bottom (highest y value)
            || self.head.position.xy.1 >= 0
        // if head has top... lowest y value? but for some reason compiler says this is wrong
        {
            self.alive = false;
        }
        match direction {
            Direction::North => {
                self.head.position.last_xy = self.head.position.xy;
                self.head.position.xy.1 -= 1; // Move north by decreasing y-coordinate

                let mut xy_prev = self.head.position.last_xy;

                for snake_block in self.body.iter_mut() {
                    let current_xy = snake_block.position.xy;
                    snake_block.position.xy = xy_prev;
                    xy_prev = current_xy;
                }

                self.length = self.body.len() as i32;
            }
            Direction::East => {
                self.head.position.last_xy = self.head.position.xy;
                self.head.position.xy.0 += 1; // Move east by increasing x-coordinate

                let mut xy_prev = self.head.position.last_xy;

                for snake_block in self.body.iter_mut() {
                    let current_xy = snake_block.position.xy;
                    snake_block.position.xy = xy_prev;
                    xy_prev = current_xy;
                }

                self.length = self.body.len() as i32;
            }
            Direction::South => {
                self.head.position.last_xy = self.head.position.xy;
                self.head.position.xy.1 += 1; // Move south by increasing y-coordinate

                let mut xy_prev = self.head.position.last_xy;

                for snake_block in self.body.iter_mut() {
                    let current_xy = snake_block.position.xy;
                    snake_block.position.xy = xy_prev;
                    xy_prev = current_xy;
                }

                self.length = self.body.len() as i32;
            }
            Direction::West => {
                self.head.position.last_xy = self.head.position.xy;
                self.head.position.xy.0 -= 1; // Move west by decreasing x-coordinate

                let mut xy_prev = self.head.position.last_xy;

                for snake_block in self.body.iter_mut() {
                    let current_xy = snake_block.position.xy;
                    snake_block.position.xy = xy_prev;
                    xy_prev = current_xy;
                }

                self.length = self.body.len() as i32;
            }
            Direction::NoDirection => {}
        }
    }
    pub fn draw_snake(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.head.draw();
        self.body.iter_mut().for_each(|element| {
            element.draw();
        });
        Ok(())
    }
}
