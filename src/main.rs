#![warn(clippy::all, clippy::pedantic)]
pub mod engine;
pub mod search_param;
pub mod search_result;
use std::{error::Error, process};
use termion::color::{self};

pub fn die(e: &dyn Error) -> ! {
    eprintln!("{}{}{}", color::Fg(color::Red), e, color::Fg(color::Reset));
    process::exit(1)
}

fn main() {
    let search_param = search_param::SearchParam::from_args();
    let mut engine = engine::Engine::from(&search_param);
    engine.search();
    dbg!(engine);
}
