use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode};

use crate::screen::draw_opening_screen;
use crate::terminal::TerminalGuard;

pub fn run(file: Option<&str>) -> io::Result<()> {
    let mut terminal = TerminalGuard::enter()?;
    let size = crossterm::terminal::size()?;

    // A abertura e a edição de arquivos ficam deliberadamente para a próxima Sprint.
    let _requested_file = file;
    draw_opening_screen(&mut terminal.stdout, size)?;

    loop {
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) if key.code == KeyCode::Esc => break,
                Event::Resize(width, height) => {
                    draw_opening_screen(&mut terminal.stdout, (width, height))?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
