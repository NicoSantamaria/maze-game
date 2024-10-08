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
    [Wall,None,Wall,None,Wall,Enem,None,None,Wall,None,Wall],
    [Wall,None,Wall,None,Wall,None,Wall,None,Wall,None,Wall],
    [Wall,None,None,None,None,None,Wall,None,None,None,Wall],
    [Wall,Wall,None,Wall,None,Wall,Wall,Wall,Wall,Wall,Wall],
    [Wall,None,None,Wall,None,Wall,None,None,None,None,Wall],
    [Wall,None,Wall,Wall,Wall,Wall,None,Wall,Wall,None,Wall],
    [Wall,None,Wall,None,None,None,None,None,Wall,None,Wall],
    [Wall,None,None,None,Wall,None,Wall,None,Wall,None,Wall],
    [Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Wall,Ends,Wall],
];

pub struct Board {
    pub stdout: io::Stdout,
    pub base: [[MazeTypes; DIMENSION]; DIMENSION],
    pub current: [[MazeTypes; DIMENSION]; DIMENSION],
    pub position_x: usize,
    pub position_y: usize,
}

impl Board {
    pub fn move_player(&mut self, next_x: usize, next_y: usize) {
        self.current[next_x][next_y] = MazeTypes::Play;
        self.current[self.position_x][self.position_y] = {
            self.base[self.position_x][self.position_y]
        };

        let _ = self.draw_pixel(next_x, next_y);
        let _ = self.draw_pixel(self.position_x, self.position_y);

        self.position_x = next_x;
        self.position_y = next_y;
    }

    pub fn draw_maze(&mut self) -> io::Result<()> {
        self.stdout.execute(terminal::Clear(terminal::ClearType::All))?;

        for x in 0..DIMENSION {
            for y in 0..DIMENSION {
                let _ = self.draw_pixel(x, y);
            }
        }

        Ok(())
    }

    pub fn draw_pixel(&mut self, x_pos: usize, y_pos: usize) -> io::Result<()> {
        let x: u16 = x_pos as u16;
        let y: u16 = y_pos as u16;
        let color = match self.current[x_pos][y_pos] {
            Strt => Color::Green,
            Ends => Color::Red,
            Wall => Color::White,
            Play => Color::Blue,
            None => Color::Black,
            Enem => Color::Red
        };

        self.stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::PrintStyledContent("█".with(color)))?
            .queue(cursor::MoveTo(0, 0))?;

        self.stdout.flush()?;
        Ok(())
    }
}