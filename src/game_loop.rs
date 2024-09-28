use std::{
    time::Duration,
    io::{self, Write},
};
use crossterm::{
    cursor, QueueableCommand,
    style::{self, Stylize, Color},
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
    let mut running: bool = true;
    let mut maze = maze::MAZE;
    let mut input: Action = Action::None;
    let mut position: [isize; 2] = [0, 1];
    let mut next_position: [isize; 2] = [0, 1];

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
                input = process_input(event);

                match input {
                    Action::Quit => running = false,
                    Action::Move(dx, dy) => {
                        let next_x = position[0] + dx as isize;
                        let next_y = position[1] + dy as isize;

                        match maze[next_x as usize][next_y as usize] {
                            maze::MazeTypes::None => next_position = [next_x, next_y], 
                            maze::MazeTypes::Enem => running = false,
                            _ => {}
                        }
                    },
                    _ => {}
                };

                stdout
                    .queue(cursor::MoveTo(position[0] as u16, position[1] as u16))?
                    .queue(style::PrintStyledContent("█".with(Color::Black)))?
                    .queue(cursor::MoveTo(next_position[0] as u16, next_position[1] as u16))?
                    .queue(style::PrintStyledContent("█".with(Color::Blue)))?;
    
                stdout.flush()?;
                position = next_position;
            }
        }
    };

    Ok(())
}

