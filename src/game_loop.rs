use std::{
    thread, time,
    io::{self, Write},
};
use crossterm::{
    cursor, QueueableCommand,
    style::{self, Stylize},
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

    // fn process_input(test: &mut u16) -> Action {
    //     if test == 5 {
    //         Action::Quit
    //     } else {
    //         Action::None
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
    
    while i < 10 {
        input = {
            if i == 5 {
                Action::Quit
            } else {
                Action::None
            }
        };
        if input == Action::Quit {
            break;
        }
        // update();
        render(stdout, &mut i)?;
        stdout.flush()?;

        thread::sleep(time::Duration::from_millis(250));
        i += 1;
    }

    Ok(())
}

