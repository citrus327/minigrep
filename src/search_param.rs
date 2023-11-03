use std::{env, process};

use termion::color::{self};

const ERROR_COLOR: color::Red = color::Red;

#[derive(Debug)]
pub struct SearchParam {
    pub search_string: String,
    pub file_path: String,
}

impl SearchParam {
    pub fn print(&self) {
        println!(
            "{}Searching keyword {} in {}",
            color::Fg(color::Blue),
            self.search_string,
            self.file_path
        );
    }
    pub fn from_args() -> Self {
        let args: Vec<String> = env::args().collect();

        if args.len() < 3 {
            eprintln!("{}Insuffcient arguments", color::Fg(ERROR_COLOR));
            process::exit(1)
        }

        let rest = &args[1..];

        let default_file_path = ".";
        let search_string = rest.get(0).map_or_else(
            || {
                eprintln!("Empty search_string, using default");
                String::new()
            },
            std::clone::Clone::clone,
        );
        let file_path = rest.get(1).map_or_else(
            || {
                eprintln!("Empty file_path, using default");
                default_file_path.to_string()
            },
            std::clone::Clone::clone,
        );

        Self {
            search_string,
            file_path,
        }
    }
}
