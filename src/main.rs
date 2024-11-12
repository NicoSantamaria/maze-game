use std::{
    time::Duration,
    io::{self},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode, KeyEvent},
};

mod board;
mod enem;
mod play;
mod maze;
mod types;

// TODO: enems only turn around when absolutely necessary
// TODO: fix bug when enems collide
// TODO: create maze = [[MazeTypes; DIMENSION]; DIMENSION] type
// TODO: extract maze and enemy generation functions?
// TODO: generally clean up code
// TODO: extract types?

fn main() -> io::Result<()> {
    let mut running: bool = true;
    let mut stage_number: u32 = 1;
    enable_raw_mode()?;

    while running {
        let mut stage_running: bool = true;
        let player: play::Play = play::Play::new(0, 1);
        let mut maze: types::MazeGrid = [
            [types::MazeTypes::Wall; types::DIMENSION]; 
        types::DIMENSION];

        maze::generate_maze(&mut maze, 1, 1);
        maze[0][1] = types::MazeTypes::Strt;
    
        let mut board_result: board::Board = board::Board::new(
            io::stdout(), 
            maze,
            player,
            maze::generate_enems(&maze, &stage_number)
        )?;

        while stage_running { 
            if let Ok(true) = poll(Duration::from_millis(250)) {
                if let Ok(event) = read() {
                    let action: types::Action = match event {
                        Event::Key(KeyEvent { code, .. }) => match code {
                            KeyCode::Char('w') | KeyCode::Up => types::Action::Move(0, -1),
                            KeyCode::Char('a') | KeyCode::Left => types::Action::Move(-1, 0),
                            KeyCode::Char('s') | KeyCode::Down => types::Action::Move(0, 1),
                            KeyCode::Char('d') | KeyCode::Right => types::Action::Move(1, 0),
                            KeyCode::Char('q') | KeyCode::Char('Q') => types::Action::Quit,
                            _ => types::Action::None,
                        },
                        _ => types::Action::None,
                    };
            
                    match action {
                        types::Action::Quit => {
                            running = false;
                            stage_running = false;
                        },
                        types::Action::Move(dx, dy) => {
                            let next_x: isize = (board_result.player.position_x as isize + dx) as isize;
                            let next_y: isize = (board_result.player.position_y as isize + dy) as isize;
            
                            if {
                                next_x >= 0 && 
                                next_x < types::DIMENSION as isize && 
                                next_y >= 0 && 
                                next_y < types::DIMENSION as isize
                            } {
                                let next_x: usize = next_x as usize;
                                let next_y: usize = next_y as usize;
            
                                match board_result.current[next_x][next_y] {
                                    types::MazeTypes::None => board_result.move_player(next_x, next_y)?,
                                    types::MazeTypes::Enem(_) => {
                                        running = false;
                                        stage_running = false;
                                    },
                                    types::MazeTypes::Ends => {
                                        stage_running = false;
                                        stage_number += 1;
                                    },
                                    _ => {}
                                }
                            }
                        },
                        _ => {}
                    }
                }
                if board_result.move_enemies()? {
                    stage_running = false;
                    running = false;
                }
            }

        }
        
    };

    disable_raw_mode()?;
    Ok(())
}
