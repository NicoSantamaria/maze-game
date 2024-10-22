use std::{
    time::Duration,
    io::{self},
};
use crossterm::{
    terminal::{enable_raw_mode, disable_raw_mode},
    event::{poll, read, Event, KeyCode, KeyEvent},
};

mod maze;
use maze::{Board, MAZE};

#[derive(PartialEq)]
enum Action {
    None,
    Quit,
    Move(isize, isize)
}

fn main() -> io::Result<()> {
    let mut running: bool = true;
    enable_raw_mode()?;

    // call function to generate maze here
    // then feed maze and enems to construct board

    let enems: Vec<(usize, usize)> = Vec::<(usize, usize)>::from([(2, 5)]);
    let mut board_result = Board::new(
        io::stdout(), 
        MAZE, 
        0, 
        1,
        enems
    )?;


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
                        let next_x: usize = (board_result.position_x as isize + dx) as usize;
                        let next_y: usize = (board_result.position_y as isize + dy) as usize;

                        match board_result.base[next_x][next_y] {
                            maze::MazeTypes::Enem => running = false,
                            maze::MazeTypes::Ends => running = false,
                            maze::MazeTypes::None => board_result.move_player(next_x, next_y)?,
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
        }
    };

    disable_raw_mode()?;
    Ok(())
}
