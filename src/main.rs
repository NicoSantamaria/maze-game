use std::{
    time::Duration,
    io::{self, Write},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode, KeyEvent},
};

mod maze;

#[derive(PartialEq)]
enum Action {
    None,
    Quit,
    Move(isize, isize)
}

struct Board {
    base: [
        [maze::MazeTypes; maze::DIMENSION]; 
    maze::DIMENSION],
    current: [
        [maze::MazeTypes; maze::DIMENSION]; 
    maze::DIMENSION],
}

impl Board {
    fn move_player(&mut self, 
        prev_x: usize, prev_y: usize,
        next_x: usize, next_y: usize
    ) {
        self.current[next_x][next_y] = maze::MazeTypes::Play;
        self.current[prev_x][prev_y] = self.base[prev_x][prev_y];
    }
}

fn main() -> io::Result<()> {
    // should change this blank declarations to actually handle errors
    let mut stdout = io::stdout();
    let mut board = Board {
        base: maze::MAZE,
        current: maze::MAZE,
    };
    
    let _ = enable_raw_mode();
    let _ = maze::draw_maze(&mut stdout);

    // let mut maze: [[maze::MazeTypes; maze::DIMENSION]; maze::DIMENSION] = maze::MAZE;
    let mut running: bool = true;
    let mut position: [usize; 2] = [0, 1];

    while running {
        if poll(Duration::from_millis(250))? {
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
                        let next_x: usize = (position[0] as isize + dx) as usize;
                        let next_y: usize = (position[1] as isize + dy) as usize;

                        match board.base[next_x][next_y] {
                            maze::MazeTypes::Enem => running = false,
                            maze::MazeTypes::Ends => running = false,
                            maze::MazeTypes::None => {
                                board.move_player(position[0], position[1], next_x, next_y);

                                let _ = maze::draw_pixel(&mut stdout, board.current, next_x, next_y);
                                let _ = maze::draw_pixel(&mut stdout, board.current, position[0], position[1]);
                                stdout.flush()?;

                                position = [next_x, next_y]
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
    };

    let _ = disable_raw_mode();

    Ok(())
}
