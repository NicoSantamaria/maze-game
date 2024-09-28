use std::io::{self, Write};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};

mod game_loop;
mod maze;

fn main() {
    // should change this blank declarations to actually handle errors
    let _ = enable_raw_mode();
    let mut stdout = io::stdout();
    let _ = maze::draw_maze(&mut stdout);
    let _ = game_loop::game_loop(&mut stdout, maze::DIMENSION);
    let _ = disable_raw_mode();
}
