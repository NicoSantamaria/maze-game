use std::io::{self, Write};
use crate::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, 
    style::{self, Stylize},
};

mod game_loop;

const DIMENSION: usize = 5;
const MAZE: [[u8; DIMENSION]; DIMENSION] = [
    [1, 0, 1, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 1, 1, 0, 1]
];

fn main() {
    // should change this blank declarations to actually handle errors
    let _ = enable_raw_mode();
    let mut stdout = io::stdout();
    let _ = draw_maze(&mut stdout);
    let _ = game_loop::game_loop(&mut stdout, DIMENSION);
    let _ = disable_raw_mode();
}

fn draw_maze(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for (x, row) in MAZE.iter().enumerate() {
        for (y, &place) in row.iter().enumerate() {
            if place == 1 {
                stdout
                    .queue(cursor::MoveTo(y as u16, x as u16))?
                    .queue(style::PrintStyledContent("█".white()))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}