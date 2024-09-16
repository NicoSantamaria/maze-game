use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    draw_maze(&mut stdout)
}

fn draw_maze(stdout: &mut io::Stdout) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    for y in 0..40 {
      for x in 0..150 {
        if (y == 0 || y == 40 - 1) || (x == 0 || x == 150 - 1) {
          // in this loop we are more efficient by not flushing the buffer.
          stdout
            .queue(cursor::MoveTo(x,y))?
            .queue(style::PrintStyledContent( "█".magenta()))?;
        }
      }
    }
    stdout.flush()?;
    Ok(())
}