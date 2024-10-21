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
    pub enems: Vec<(usize, usize)>
}

impl Board {
    pub fn new(
        mut stdout: io::Stdout,
        base: [[MazeTypes; DIMENSION]; DIMENSION],
        position_x: usize,
        position_y: usize,
    ) -> Result<Self, io::Error> {
        let _ = stdout
            .execute(terminal::Clear(terminal::ClearType::All))?;

        let mut enemies = Vec::<(usize, usize)>::new();

        for x in 0..DIMENSION {
            for y in 0..DIMENSION {
                let _ = Board::draw_pixel(&stdout, x, y, &base)?; // Fix this line
                if base[x][y] == MazeTypes::Enem {
                    enemies.push((x, y));
                }
            }
        }

        Ok(Board {
            stdout,
            base,
            current: base.clone(),
            position_x,
            position_y,
            enems: enemies,
        })
    }

    pub fn move_player(&mut self, next_x: usize, next_y: usize) -> io::Result<()> {
        self.current[next_x][next_y] = MazeTypes::Play;
        self.current[self.position_x][self.position_y] = {
            self.base[self.position_x][self.position_y]
        };

        Board::draw_pixel(&self.stdout, next_x, next_y, &self.current)?;
        Board::draw_pixel(&self.stdout, self.position_x, self.position_y, &self.current)?;

        self.position_x = next_x;
        self.position_y = next_y;

        Ok(())
    }

    pub fn move_enemies(&mut self) -> io::Result<()> {

        Ok(())
    }

    pub fn draw_pixel(
        mut stdout: &io::Stdout,
        x_pos: usize, 
        y_pos: usize, 
        maze: &[[MazeTypes; DIMENSION]; DIMENSION]
    ) -> io::Result<()> {
        let board = &maze;
        let x: u16 = x_pos as u16;
        let y: u16 = y_pos as u16;
        let color = match board[x_pos][y_pos] {
            Strt => Color::Green,
            Ends => Color::Red,
            Wall => Color::White,
            Play => Color::Blue,
            None => Color::Black,
            Enem => Color::Red
        };

        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::PrintStyledContent("█".with(color)))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;

        Ok(())
    }
}