use std::{
    time::Duration,
    io::{self, Write},
};
use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
};
use crate::maze;

#[derive(PartialEq)]
enum Action {
    None,
    Quit,
    Move(isize, isize)
}

pub fn game_loop(stdout: &mut io::Stdout) -> io::Result<()> {
    let mut maze: [[maze::MazeTypes; maze::DIMENSION]; maze::DIMENSION] = maze::MAZE;
    let mut running: bool = true;
    let mut position: [usize; 2] = [0, 1];

    fn process_input(event: Event) -> Action {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char('w') | KeyCode::Up => Action::Move(0, -1),
                KeyCode::Char('a') | KeyCode::Left => Action::Move(-1, 0),
                KeyCode::Char('s') | KeyCode::Down => Action::Move(0, 1),
                KeyCode::Char('d') | KeyCode::Right => Action::Move(1, 0),
                KeyCode::Char('q') | KeyCode::Char('Q') => Action::Quit,
                _ => Action::None,
            },
            _ => Action::None,
        }
    }
    
    while running {
        if poll(Duration::from_millis(250))? {
            if let Ok(event) = read() {
                match process_input(event) {
                    Action::Quit => running = false,
                    Action::Move(dx, dy) => {
                        let next_x: usize = (position[0] as isize + dx) as usize;
                        let next_y: usize = (position[1] as isize + dy) as usize;

                        match maze::MAZE[next_x][next_y] {
                            maze::MazeTypes::Enem => running = false,
                            maze::MazeTypes::Ends => running = false,
                            maze::MazeTypes::None => {
                                maze[next_x][next_y] = maze::MazeTypes::Play;
                                let maze_copy = maze;

                                let _ = maze::draw_pixel(stdout, maze_copy, next_x, next_y);
                                let _ = maze::draw_pixel(stdout, maze::MAZE, position[0], position[1]);
                                stdout.flush()?;

                                position = [next_x, next_y]
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                };
            }
        }
    };

    Ok(())
}

