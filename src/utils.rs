use std::{error::Error, process};

use termion::color;

pub fn die(e: &dyn Error) -> ! {
    eprintln!("{}{}{}", color::Fg(color::Red), e, color::Fg(color::Reset));
    process::exit(1)
}
