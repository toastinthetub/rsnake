use crossterm::{
    cursor::{self, MoveTo},
    execute,
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::{
    error::Error,
    io::{Stdout, Write},
};

pub const ANSII_RED: &str = "\x1b[1;31m";
// other ansii colors soon

pub const ANSII_RESET: &str = "\x1b[0m";

const BOX_CHAR: &str = "█";

pub enum AnsiiColor {
    Red,
    // more soon
    Reset,
}

pub enum IsVertical {
    Yes,
    No,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ScreenConstruct<'a> {
    pub w: u16,
    pub h: u16,

    pub bar_char: String,

    pub bnr_color_on: &'a str,
    pub bnr_color_reset: &'a str,
}

impl ScreenConstruct<'_> {
    pub fn new() -> Result<(Self, Stdout), Box<dyn Error>> {
        // creates and empty ScreenConstruct
        // defaults; 50x50, "0", default term, default term
        let stdout = std::io::stdout();

        Ok((
            Self {
                w: 50,
                h: 50,

                bar_char: String::from("0"),

                bnr_color_on: ANSII_RESET,
                bnr_color_reset: ANSII_RESET,
            },
            stdout,
        ))
    }
    // alternatively you can construct the screen directly ...
    pub fn construct_scr(
        bnr_color_on: AnsiiColor,
        bnr_color_reset: AnsiiColor,
        mut bar_char: Option<String>,
    ) -> Result<(Self, Stdout), Box<dyn Error>> {
        let (w, h) = crossterm::terminal::size().expect("Failed to get terminal size!");
        let stdout = std::io::stdout();
        if bar_char.is_some() {
        } else {
            bar_char = Some(String::from("█"));
        }
        let bnr_color_on = match bnr_color_on {
            AnsiiColor::Red => ANSII_RED,
            AnsiiColor::Reset => ANSII_RESET,
        };
        let bnr_color_reset = match bnr_color_reset {
            AnsiiColor::Red => ANSII_RED,
            AnsiiColor::Reset => ANSII_RESET,
        };

        Ok((
            Self {
                w,
                h,
                bar_char: bar_char.unwrap(), // i will not be judged for unwrapping!
                bnr_color_on,
                bnr_color_reset,
            },
            stdout,
        ))
    }
    pub fn init_scr(
        &mut self,
        bnr_color_on: AnsiiColor,
        bnr_color_reset: AnsiiColor,
        mut bar_char: Option<String>,
    ) -> Result<(), Box<dyn Error>> {
        let (w, h) = crossterm::terminal::size().expect("Failed to get terminal size!");
        if bar_char.is_some() {
        } else {
            bar_char = Some(String::from("█"));
        }
        let bnr_color_on = match bnr_color_on {
            AnsiiColor::Red => ANSII_RED,
            AnsiiColor::Reset => ANSII_RESET,
        };
        let bnr_color_reset = match bnr_color_reset {
            AnsiiColor::Red => ANSII_RED,
            AnsiiColor::Reset => ANSII_RESET,
        };

        (self.w, self.h) = (w, h);
        self.bnr_color_on = bnr_color_on;
        self.bnr_color_reset = bnr_color_reset;
        self.bar_char = bar_char.unwrap(); // i know :|

        Ok(())
    }
}

impl ScreenConstruct<'_> {
    pub fn prime_scr(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        let _ = crossterm::terminal::enable_raw_mode();
        let _ = execute!(stdout, cursor::DisableBlinking);
        let _ = stdout.queue(Clear(ClearType::All));
        let _ = stdout.flush().unwrap();
        Ok(())
    }
    pub fn sieze_scr(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        let _ = stdout.queue(Clear(ClearType::All));
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(stdout, cursor::EnableBlinking);
        Ok(())
    }
    pub fn update_screen_size(&mut self) -> Result<(), Box<dyn Error>> {
        (self.w, self.h) = crossterm::terminal::size()?;
        Ok(())
    }
    // lots of booooilerplate.
    pub fn create_bar(
        &mut self,
        stdout: &mut Stdout,
        vertical: IsVertical,
        double: bool,
        mut char: String,
        x: u16,
        start_y: Option<u16>,
        end: Option<u16>,
        line: Option<u16>,
    ) -> Result<(), ()> {
        match double {
            true => {
                char = format!(
                    "{}{}{}{}",
                    self.bnr_color_on, char, char, self.bnr_color_reset
                )
            }
            false => {
                char = format!("{}{}{}", self.bnr_color_on, char, self.bnr_color_reset);
                // format char to correct colors
            }
        }
        match vertical {
            IsVertical::Yes => {
                if start_y.is_none() && end.is_none() {
                    let _ = stdout.queue(MoveTo(x, 0));
                    for cell in 0..=self.h {
                        // bar length is just the height in this case
                        let _ = stdout.write(char.as_bytes());
                        let _ = stdout.queue(MoveTo(x, cell));
                        let _ = stdout.flush();
                    }
                    Ok(())
                } else {
                    let _ = stdout.queue(MoveTo(x, start_y.unwrap()));
                    let length = end.unwrap() - start_y.unwrap();
                    for cell in 0..=length {
                        let _ = stdout.write(char.as_bytes());
                        let _ = stdout.queue(MoveTo(x, cell));
                        let _ = stdout.flush();
                    }
                    Ok(())
                }
            }
            IsVertical::No => {
                match double {
                    true => {
                        if line.is_some() {
                            eprintln!("\x1b[1;31m;You tried to use a double character in a horizontal bar on line {}, are you sure you wanted to do that?\x1b[0m", line.unwrap());
                        } else {
                            eprintln!("\x1b[1;31m;You tried to use a double character in a horizontal bar are you sure you wanted to do that?\x1b[0m");
                        }
                    }
                    false => { /* do nothing */ }
                }
                if start_y.is_some() && end.is_some() {
                    let _ = stdout.queue(MoveTo(x, start_y.unwrap()));
                    let length = end.unwrap() - x;
                    for cell in 0..=length {
                        let _ = stdout.write(char.as_bytes());
                        let _ = stdout.queue(MoveTo(cell, start_y.unwrap()));
                        let _ = stdout.flush();
                    }
                } else {
                    let _ = stdout.queue(MoveTo(x, start_y.unwrap()));
                    for cell in 0..self.w {
                        // bar length is just the width in this case
                        let _ = stdout.write(char.as_bytes());
                        let _ = stdout.queue(MoveTo(cell, start_y.unwrap()));
                        let _ = stdout.flush();
                    }
                }
                Ok(())
            }
        }
    }
}

// built in, fast, generic UI elements
impl ScreenConstruct<'_> {
    pub fn create_border(&mut self, stdout: &mut Stdout) -> Result<(), Box<dyn Error>> {
        self.create_bar(
            // left vertical
            stdout,
            IsVertical::Yes,
            true,
            String::from(BOX_CHAR),
            0,
            Some(0),
            Some(self.h),
            Some(232),
        )
        .unwrap();
        self.create_bar(
            // bottom bar
            stdout,
            IsVertical::No,
            false,
            String::from(BOX_CHAR),
            0,
            Some(self.h),
            Some(self.w), // this is fucking confusing why did i do this
            Some(243),
        )
        .unwrap();
        self.create_bar(
            // top bar
            stdout,
            IsVertical::No,
            false,
            String::from(BOX_CHAR),
            0,
            Some(0),
            Some(self.w),
            Some(254),
        )
        .unwrap();
        self.create_bar(
            //rightmost vertical bar
            stdout,
            IsVertical::Yes,
            true,
            String::from(BOX_CHAR),
            self.w - 2,
            Some(0),
            Some(self.h),
            Some(265),
        )
        .unwrap();
        Ok(())
    }
}
