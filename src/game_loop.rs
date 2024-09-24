use std::{
    thread, time,
    io::{self, Write},
};
use crossterm::{
    cursor, QueueableCommand,
    style::{self, Stylize},
};
// #[derive(PartialEq)]
// enum Action {
//     None,
//     Left,
//     Right,
//     Up,
//     Down,
//     Quit
// }


pub fn game_loop(stdout: &mut io::Stdout) -> io::Result<()> {
    // let mut input = Action::None;

    // fn process_input(test: &mut u16) -> Action {
    //     if test == 5 {
    //         Action::Quit
    //     } else {
    //         Action::None
    //     }
        // let event = read()?;

        // if event == Event::Key(KeyCode::Char('q'))
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

    let mut i: u16 = 0;
    while i < 10 {
        // input = process_input(&mut i);
        // if input == Action::Quit {
        //     break;
        // }
        // update();
        render(stdout, &mut i)?;
        stdout.flush()?;

        thread::sleep(time::Duration::from_millis(250));
        i += 1;
    }

    Ok(())
}

