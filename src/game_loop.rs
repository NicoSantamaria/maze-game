use std::{
    thread, time::Duration,
    io::{self, Write},
};
use crossterm::{
    cursor, QueueableCommand,
    style::{self, Stylize},
    event::{poll, read, Event, KeyCode},
};

#[derive(PartialEq)]
enum Action {
    None,
    Left,
    Right,
    Up,
    Down,
    Quit
}


pub fn game_loop(stdout: &mut io::Stdout) -> io::Result<()> {
    let mut running = true;
    let mut input = Action::None;
    let mut i: u16 = 0;

    // fn process_input() {
    //     let event = read()?;

    //     if event == Event::Key(KeyCode::Char('q').into()) {
    //         return Ok(Action::Quit)
    //     } else {
    //         return Ok(Action::None)
    //     }
    // }

    // fn update() {
    // }

    fn render(stdout: &mut io::Stdout, y: &mut u16) -> io::Result<()> {
        let coordinate = *y;
        stdout
            .queue(cursor::MoveTo(coordinate, coordinate))?
            .queue(style::PrintStyledContent( "█".magenta()))?;

        Ok(())
    }
    
    while running {
        if poll(Duration::from_millis(250))? {
            let event = read()?;
            if event == Event::Key(KeyCode::Char('q').into()) {
                input = Action::Quit;
            } else {
                input = Action::None;
            };
        }

        // input = process_input();
        // update();
        render(stdout, &mut i)?;

        stdout.flush()?;

        // thread::sleep(time::Duration::from_millis(250));
        i += 1;

        if input == Action::Quit || i == 10 {
            running = false;
        }
    };

    Ok(())
}

