use std::io::{self, Write};
use crossterm::{
    // ExecutableCommand, 
    QueueableCommand,
    // terminal, 
    cursor, 
    style::{self, Stylize, Color},
};
use crate::{play, enem, MazeTypes, DIMENSION};

pub struct Board {
    pub stdout: io::Stdout,
    pub base: [[MazeTypes; DIMENSION]; DIMENSION],
    pub current: [[MazeTypes; DIMENSION]; DIMENSION],
    pub player: play::Play,
    pub enems: Vec<enem::Enem>
}

impl Board {
    pub fn new(
        stdout: io::Stdout,
        base: [[MazeTypes; DIMENSION]; DIMENSION],
        player: play::Play,
        enems: Vec<enem::Enem>
    ) -> Result<Self, io::Error> {
        let mut current: [[MazeTypes; DIMENSION]; DIMENSION] = base.clone();
        current[0][1] = MazeTypes::Play(player);
        current[DIMENSION - 1][DIMENSION - 2] = MazeTypes::Ends;

        for x in 0..DIMENSION {
            for y in 0..DIMENSION {
                let candidate: enem::Enem = enem::Enem::new(x, y);
                if enems.contains(&candidate) {
                    current[x][y] = MazeTypes::Enem(candidate);
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

    pub fn move_enemies(&mut self) -> Result<bool, io::Error> {
        for enemy in self.enems.iter_mut() {
            let mut running: bool = true;
    
            while running {
                if enemy.position_x == self.player.position_x && enemy.position_y == self.player.position_y {
                    return Ok(true);
                }

                let dx = enemy.last_move.0;
                let dy = enemy.last_move.1;
                let next_x = (enemy.position_x as isize + dx) as isize;
                let next_y = (enemy.position_y as isize + dy) as isize;
    
                if next_x >= 0 && next_x < DIMENSION as isize && next_y >= 0 && next_y < DIMENSION as isize {
                    let next_x = next_x as usize;
                    let next_y = next_y as usize;
    
                    match self.current[next_x][next_y] {
                        MazeTypes::None => {
                            self.current[next_x][next_y] = MazeTypes::Enem(*enemy);
                            self.current[enemy.position_x][enemy.position_y] = {
                                self.base[enemy.position_x][enemy.position_y]
                            };
    
                            Board::draw_pixel(&self.stdout, next_x, next_y, &self.current)?;
                            Board::draw_pixel(&self.stdout, enemy.position_x, enemy.position_y, &self.current)?;
    
                            enemy.position_x = next_x;
                            enemy.position_y = next_y;
                            enemy.last_move = (dx, dy);
    
                            running = false;
                        },
                        MazeTypes::Play(_) => {
                            return Ok(true)
                        }
                        _ => {
                            enemy.last_move = enem::Enem::new_move();
                        }
                    }
                } else {
                    enemy.last_move = enem::Enem::new_move();
                }
            }
        }
    
        Ok(false)
    }

    pub fn draw_pixel(
        mut stdout: &io::Stdout,
        x_pos: usize, 
        y_pos: usize, 
        maze: &[[MazeTypes; DIMENSION]; DIMENSION]
    ) -> io::Result<()> {
        let board: &&[[MazeTypes; DIMENSION]; DIMENSION] = &maze;
        let x: u16 = x_pos as u16;
        let y: u16 = y_pos as u16;
        let color: Color = match board[x_pos][y_pos] {
            MazeTypes::Strt => Color::Green,
            MazeTypes::Ends => Color::Red,
            MazeTypes::Wall => Color::White,
            MazeTypes::Play(_) => Color::Blue,
            MazeTypes::Enem(_) => Color::Red,
            MazeTypes::None => Color::Black
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