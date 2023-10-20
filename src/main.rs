#![warn(clippy::all, clippy::pedantic)]
pub mod engine;
pub mod search_param;
pub mod search_result;

mod utils;

fn main() {
    let search_param = search_param::SearchParam::from_args();
    let mut engine = engine::Engine::from(&search_param);
    engine.search();
    dbg!(engine);
}
