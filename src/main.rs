mod game;
mod listener;
mod snake;
mod utils;

use crossterm::{cursor::MoveTo, QueueableCommand};
use std::io::{Stdout, Write};

use crate::{
    game::{Game, IsQuit},
    snake::Snake,
    utils::{AnsiiColor, ScreenConstruct, ANSII_RED, ANSII_RESET},
};

const BOX_CHAR: &str = "█";

fn main() -> Result<(), ()> {
    // you can also ```

    // let (mut screen, mut stdout) = ScreenConstruct::new();
    // sceen.init_scr(AnsiiColor::Color, AnsiiColor::Color, bar_char: "C")

    // ``` I chose to construct the screen directly

    let (mut screen, mut stdout) =
        ScreenConstruct::construct_scr(AnsiiColor::Red, AnsiiColor::Reset, Some(String::from("0")))
            .unwrap();

    let _ = screen.prime_scr(&mut stdout);
    let snake = Snake::new();
    let mut game = Game::instantiate_game(screen.clone(), snake);

    let mut quit: bool = false;

    while !quit {
        match game.listener(&mut stdout) {
            Ok(quitstatus) => {
                match quitstatus {
                    IsQuit::Quit => {
                        let _ = screen.sieze_scr(&mut stdout);
                        quit = true;
                    }
                    IsQuit::DontQuit => {
                        // do nothing
                    }
                }
            }
            Err(e) => {
                eprintln!("\x1b[1;31m[]ERROR OCCURED: {}\x1b[0m", e)
            }
        }
    }
    Ok(())
}

pub fn draw_cell_arbitrary(stdout: &mut Stdout, color: AnsiiColor, position: (u16, u16)) {
    let character = format!(
        "{}{}{}",
        match color {
            AnsiiColor::Red => {
                ANSII_RED
            }
            _ => {
                ANSII_RED // ansii red for now
            }
        },
        BOX_CHAR,
        ANSII_RESET
    );
    let _ = stdout.queue(MoveTo(position.0, position.1));
    let _ = stdout.write(character.as_bytes());
    let _ = stdout.flush();
}
