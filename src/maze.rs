use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, 
    style::{self, Stylize},
};

pub const DIMENSION: usize = 5;
pub const MAZE: [[u8; DIMENSION]; DIMENSION] = [
    [1, 0, 1, 1, 1],
    [1, 0, 1, 0, 1],
    [1, 0, 0, 0, 1],
    [1, 0, 1, 0, 1],
    [1, 1, 1, 0, 1]
];

pub fn draw_maze(stdout: &mut io::Stdout) -> io::Result<()> {
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