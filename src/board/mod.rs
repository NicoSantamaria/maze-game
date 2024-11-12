use std::io::{self, Write};
use crossterm::{
    // ExecutableCommand, 
    QueueableCommand,
    // terminal, 
    cursor, 
    style::{self, Stylize, Color},
};
use crate::{play, enem, types};

pub struct Board {
    pub stdout: io::Stdout,
    pub base: types::MazeGrid,
    pub current: types::MazeGrid,
    pub player: play::Play,
    pub enems: Vec<enem::Enem>
}

impl Board {
    pub fn new(
        stdout: io::Stdout,
        base: types::MazeGrid,
        player: play::Play,
        enems: Vec<enem::Enem>
    ) -> Result<Self, io::Error> {
        let mut current: types::MazeGrid = base.clone();
        current[0][1] = types::MazeTypes::Play(player);
        current[types::DIMENSION - 1][types::DIMENSION - 2] = types::MazeTypes::Ends;

        for x in 0..types::DIMENSION {
            for y in 0..types::DIMENSION {
                let candidate: enem::Enem = enem::Enem::new(x, y);
                if enems.contains(&candidate) {
                    current[x][y] = types::MazeTypes::Enem(candidate);
                }

                Board::draw_pixel(&stdout, x, y, &current)?;
            }
        }

        Ok(Board { 
            stdout, 
            base, 
            current, 
            player,
            enems,
        })
    }

    pub fn move_player(
        &mut self, 
        next_x: usize, 
        next_y: usize,
    ) -> io::Result<()> {
        self.current[next_x][next_y] = types::MazeTypes::Play(self.player);
        self.current[self.player.position_x][self.player.position_y] = {
            self.base[self.player.position_x][self.player.position_y]
        };

        Board::draw_pixel(&self.stdout, next_x, next_y, &self.current)?;
        Board::draw_pixel(&self.stdout, self.player.position_x, self.player.position_y, &self.current)?;

        self.player.position_x = next_x;
        self.player.position_y = next_y;

        Ok(())
    }

    pub fn move_enemies(&mut self) -> Result<bool, io::Error> {
        for enemy in self.enems.iter_mut() {
            // Check for collision with player before movement
            if enemy.position_x == self.player.position_x && 
                enemy.position_y == self.player.position_y {
                return Ok(true);
            }

            let ((dx, dy), (next_x, next_y)) = enemy.new_move(&self.current);

            // Update board state for enemy movement
            match self.current[next_x][next_y] {
                types::MazeTypes::None => {
                    // Clear old position
                    self.current[enemy.position_x][enemy.position_y] = 
                        self.base[enemy.position_x][enemy.position_y];
                    
                    // Set new position
                    self.current[next_x][next_y] = types::MazeTypes::Enem(*enemy);

                    // Update display
                    Board::draw_pixel(&self.stdout, next_x, next_y, &self.current)?;
                    Board::draw_pixel(
                        &self.stdout, 
                        enemy.position_x, 
                        enemy.position_y, 
                        &self.current
                    )?;

                    enemy.position_x = next_x;
                    enemy.position_y = next_y;
                },
                types::MazeTypes::Play(_) => return Ok(true),
                _ => {},
            }
        }
        Ok(false)
    }

    pub fn draw_pixel(
        mut stdout: &io::Stdout,
        x_pos: usize, 
        y_pos: usize, 
        maze: &types::MazeGrid
    ) -> io::Result<()> {
        let board: &&types::MazeGrid = &maze;
        let x: u16 = x_pos as u16;
        let y: u16 = y_pos as u16;
        let color: Color = match board[x_pos][y_pos] {
            types::MazeTypes::Strt => Color::Green,
            types::MazeTypes::Ends => Color::Red,
            types::MazeTypes::Wall => Color::White,
            types::MazeTypes::Play(_) => Color::Blue,
            types::MazeTypes::Enem(_) => Color::Red,
            types::MazeTypes::None => Color::Black
        };

        stdout
            .queue(cursor::MoveTo(x, y))?
            .queue(style::PrintStyledContent("█".with(color)))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;

        Ok(())
    }
}