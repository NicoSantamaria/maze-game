use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, 
    style::{self, Stylize, Color},
};

#[derive(Copy, Clone, PartialEq)]
pub enum MazeTypes {
    Strt,
    Ends,
    Wall,
    Play,
    None,
    Enem,
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

pub fn draw_pixel(stdout: &mut io::Stdout, x_pos: usize, y_pos: usize) -> io::Result<()> {
    let x: u16 = x_pos as u16;
    let y: u16 = y_pos as u16;
    let color = match MAZE[x_pos][y_pos] {
        Strt => Color::Green,
        Ends => Color::Red,
        Wall => Color::White,
        Play => Color::Blue,
        None => Color::Black,
        Enem => Color::Red
    };

    stdout
        .queue(cursor::MoveTo(x, y))?
        .queue(style::PrintStyledContent("█".with(color)))?;

    Ok(())
}

pub fn draw_maze(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for x in 0..DIMENSION {
        for y in 0..DIMENSION {
            let _ = draw_pixel(stdout, x, y);
        }
    }

    stdout.flush()?;
    Ok(())
}