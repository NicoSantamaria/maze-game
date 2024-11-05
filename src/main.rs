use std::{
    time::Duration,
    io::{self},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode, KeyEvent},
};
use rand::{
    seq::SliceRandom,
    thread_rng
};

mod board;
mod enem;
mod play;

#[derive(Copy, Clone, PartialEq)]
pub enum MazeTypes {
    Strt,
    Ends,
    Wall,
    Play(play::Play),
    Enem(enem::Enem),
    None
}

#[derive(PartialEq)]
enum Action {
    None,
    Quit,
    Move(isize, isize)
}

use MazeTypes::*;

const DIMENSION: usize = 37;

fn main() -> io::Result<()> {
    fn generate_maze(maze: &mut [[MazeTypes; DIMENSION]; DIMENSION], x: usize, y: usize, target_x: usize, target_y: usize) {
        let directions: [(isize, isize); 4] = [(0, 2), (0, -2), (2, 0), (-2, 0)];
        let mut rng: rand::prelude::ThreadRng = thread_rng();
        let mut shuffled_directions: Vec<(isize, isize)> = directions.to_vec();
        shuffled_directions.shuffle(&mut rng);
    
        maze[x][y] = None;
    
        for &(dx, dy) in &shuffled_directions {
            let next_x: usize = (x as isize + dx) as usize;
            let next_y: usize = (y as isize + dy) as usize;
    
            if next_x < DIMENSION && next_y < DIMENSION && maze[next_x][next_y] == Wall {
                let x_coord: usize = (x as isize + dx / 2) as usize;
                let y_coord: usize = (y as isize + dy / 2) as usize;

                maze[x_coord][y_coord] = match (x_coord, y_coord) {
                    (x, y) if x == target_x && y == target_y => Ends,
                    _ => None,
                };

                generate_maze(maze, next_x, next_y, target_x, target_y);
            }
        }
    }

    let mut running: bool = true;
    enable_raw_mode()?;
    
    let player: play::Play = play::Play::new(0, 1);
    let mut maze: [[MazeTypes; DIMENSION]; DIMENSION] = [[Wall; DIMENSION]; DIMENSION];
    generate_maze(&mut maze, 1, 1, DIMENSION - 1, DIMENSION - 2);

    let enems: Vec<enem::Enem> = Vec::<enem::Enem>::from([
        enem::Enem::new(2, 5),
        enem::Enem::new(5, 2),
        enem::Enem::new(9, 9),
    ]);
    let mut board_result: board::Board = board::Board::new(
        io::stdout(), 
        maze, 
        player,
        enems
    )?;


    while running {
        if let Ok(true) = poll(Duration::from_millis(250)) {
            if let Ok(event) = read() {
                let action: Action = match event {
                    Event::Key(KeyEvent { code, .. }) => match code {
                        KeyCode::Char('w') | KeyCode::Up => Action::Move(0, -1),
                        KeyCode::Char('a') | KeyCode::Left => Action::Move(-1, 0),
                        KeyCode::Char('s') | KeyCode::Down => Action::Move(0, 1),
                        KeyCode::Char('d') | KeyCode::Right => Action::Move(1, 0),
                        KeyCode::Char('q') | KeyCode::Char('Q') => Action::Quit,
                        _ => Action::None,
                    },
                    _ => Action::None,
                };
        
                match action {
                    Action::Quit => running = false,
                    Action::Move(dx, dy) => {
                        let next_x: isize = (board_result.player.position_x as isize + dx) as isize;
                        let next_y: isize = (board_result.player.position_y as isize + dy) as isize;
        
                        if next_x >= 0 && next_x < DIMENSION as isize && next_y >= 0 && next_y < DIMENSION as isize {
                            let next_x: usize = next_x as usize;
                            let next_y: usize = next_y as usize;
        
                            match board_result.base[next_x][next_y] {
                                MazeTypes::Enem(_) | MazeTypes::Ends => running = false,
                                MazeTypes::None => board_result.move_player(next_x, next_y)?,
                                _ => {}
                            }
                        }
                    },
                    _ => {}
                }
            }
            if board_result.move_enemies()? {
                running = false;
            }
        }
        
    };

    disable_raw_mode()?;
    Ok(())
}
