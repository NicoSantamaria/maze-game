use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, 
    style::{self, Stylize, Color},
};

#[derive(Copy, Clone)]
pub enum MazeTypes {
    Strt,
    Ends,
    Wall,
    Play,
    None,
    // Enem,
}

use MazeTypes::*;

pub const DIMENSION: usize = 11;
pub const MAZE: [[MazeTypes; DIMENSION]; DIMENSION] = [
    [Wall,Strt,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall],
    [Wall,None,Wall,None,Wall,None,Wall,None,Wall,None,Wall],
    [Wall,None,Wall,None,Wall,None,None,None,Wall,None,Wall],
    [Wall,None,Wall,None,Wall,None,Wall,None,Wall,None,Wall],
    [Wall,None,None,None,None,None,Wall,None,None,None,Wall],
    [Wall,Wall,None,Wall,None,Wall,Wall,Wall,Wall,Wall,Wall],
    [Wall,None,None,Wall,None,Wall,None,None,None,None,Wall],
    [Wall,None,Wall,Wall,Wall,Wall,None,Wall,Wall,None,Wall],
    [Wall,None,Wall,None,None,None,None,None,Wall,None,Wall],
    [Wall,None,None,None,Wall,None,Wall,None,Wall,None,Wall],
    [Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Ends,Wall],
];

pub fn draw_maze(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for (x, row) in MAZE.iter().enumerate() {
        for (y, &place) in row.iter().enumerate() {
            let color = match place {
                Strt => Color::Green,
                Ends => Color::Red,
                Wall => Color::White,
                Play => Color::Blue,
                None => Color::Black,
            };

            stdout
                .queue(cursor::MoveTo(y as u16, x as u16))?
                .queue(style::PrintStyledContent("█".with(color)))?;
        }
    }

    stdout.flush()?;
    Ok(())
}