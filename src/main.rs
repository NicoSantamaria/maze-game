use std::io::{self, Write};
use std::{thread, time};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

fn main() {
    let mut stdout = io::stdout();
    draw_maze(&mut stdout);
    game_loop(&mut stdout);
}

fn draw_maze(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    
    const DIMENSION: usize = 5;
    const MAZE: [[u8; DIMENSION]; DIMENSION] = [
        [1, 0, 1, 1, 1],
        [1, 0, 1, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 1, 0, 1],
        [1, 1, 1, 0, 1]
    ];

    for (x, row) in MAZE.iter().enumerate() {
        for (y, &place) in row.iter().enumerate() {
            if place == 1 {
                stdout
                    .queue(cursor::MoveTo(y as u16, x as u16))?
                    .queue(style::PrintStyledContent("█".white()))?;
            }
        }
    }

    stdout.flush()?;
    Ok(())
}

fn game_loop(stdout: &mut io::Stdout) -> io::Result<()> {
    // fn process_input() {
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
        // process_input();
        // update();
        render(stdout, &mut i)?;
        stdout.flush()?;

        thread::sleep(time::Duration::from_millis(250));
        i += 1;
    }

    Ok(())
}