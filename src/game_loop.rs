use std::{
    time::Duration,
    io::{self, Write},
};
use crossterm::{
    cursor, QueueableCommand,
    style::{self, Stylize},
    event::{poll, read, Event, KeyCode, KeyEvent},
};

#[derive(PartialEq)]
enum Action {
    None,
    Quit,
    Move(i8, i8)
}


pub fn game_loop(stdout: &mut io::Stdout, dim: usize) -> io::Result<()> {
    let mut running: bool = true;
    let mut input: Action = Action::None;
    let mut position: [u8; 2] = [0, 0];

    fn process_input(event: Event) -> Action {
        match event {
            Event::Key(KeyEvent { code, .. }) => match code {
                KeyCode::Char('w') => Action::Move(0, 1),
                KeyCode::Char('a') => Action::Move(-1, 0),
                KeyCode::Char('s') => Action::Move(0, -1),
                KeyCode::Char('d') => Action::Move(1, 0),
                KeyCode::Char('q') => Action::Quit,
                _ => Action::None,
            },
            _ => Action::None,
        }
    }

    // fn update() {
    // }

    // fn render(stdout: &mut io::Stdout, y: &mut u16) -> io::Result<()> {
    //     let coordinate = *y;
    //     stdout
    //         .queue(cursor::MoveTo(coordinate, coordinate))?
    //         .queue(style::PrintStyledContent( "█".magenta()))?;

    //     Ok(())
    // }
    
    while running {
        if poll(Duration::from_millis(250))? {
            if let Ok(event) = read() {
                input = process_input(event);

                match input {
                    Action::Quit => running = false,
                    Action:: None => {},
                    Action::Move(dx, 0) => {
                        let next_position: i8 = position[0] as i8 + dx;
                        if next_position > -1 && next_position < dim as i8 {
                            position[0] = next_position as u8;
                        }
                    },
                    Action::Move(0, dy) => {
                        let next_position: i8 = position[1] as i8 + dy;
                        if next_position > -1 && next_position < dim as i8  {
                            position[1] = next_position as u8;
                        }
                    },
                    _ => {}
                }
            }
        }

        // render(stdout, &mut i)?;
        // stdout.flush()?;
    };

    Ok(())
}

