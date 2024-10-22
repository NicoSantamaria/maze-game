use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, 
    style::{self, Stylize, Color},
};
use crate::{play, MazeTypes, DIMENSION};

pub struct Board {
    pub stdout: io::Stdout,
    pub base: [[MazeTypes; DIMENSION]; DIMENSION],
    pub current: [[MazeTypes; DIMENSION]; DIMENSION],
    pub player: play::Play
}

impl Board {
    pub fn new(
        stdout: io::Stdout,
        base: [[MazeTypes; DIMENSION]; DIMENSION],
        player: play::Play,
        enems: Vec<(usize, usize)>
    ) -> Result<Self, io::Error> {
        let mut current: [[MazeTypes; 11]; 11] = base.clone();

        for x in 0..DIMENSION {
            for y in 0..DIMENSION {
                if enems.contains(&(x, y)) {
                    current[x][y] = MazeTypes::Enem;
                }

                Board::draw_pixel(&stdout, x, y, &current)?;
            }
        }

        Ok(Board { 
            stdout, 
            base, 
            current, 
            player,
        })
    }

    pub fn move_player(
        &mut self, 
        next_x: usize, 
        next_y: usize,
    ) -> io::Result<()> {
        self.current[next_x][next_y] = MazeTypes::Play(self.player);
        self.current[self.player.position_x][self.player.position_y] = {
            self.base[self.player.position_x][self.player.position_y]
        };

        Board::draw_pixel(&self.stdout, next_x, next_y, &self.current)?;
        Board::draw_pixel(&self.stdout, self.player.position_x, self.player.position_y, &self.current)?;

        self.player.position_x = next_x;
        self.player.position_y = next_y;

        Ok(())
    }

    // pub fn move_enemy() -> io::Result<()> {
    //     Ok(())
    // }

    // pub fn enem_next_move() {

    // }

    pub fn draw_pixel(
        mut stdout: &io::Stdout,
        x_pos: usize, 
        y_pos: usize, 
        maze: &[[MazeTypes; DIMENSION]; DIMENSION]
    ) -> io::Result<()> {
        let board: &&[[MazeTypes; 11]; 11] = &maze;
        let x: u16 = x_pos as u16;
        let y: u16 = y_pos as u16;
        let color: Color = match board[x_pos][y_pos] {
            MazeTypes::Strt => Color::Green,
            MazeTypes::Ends => Color::Red,
            MazeTypes::Wall => Color::White,
            MazeTypes::Play(_) => Color::Blue,
            MazeTypes::None => Color::Black,
            MazeTypes::Enem => Color::Red
        };

        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::PrintStyledContent("█".with(color)))?
            // moving back to the origin might be causing more problems...
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;

        Ok(())
    }
}