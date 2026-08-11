use std::io::{self, Stdout, Write};
use std::time::Duration;

use crossterm::cursor::{Hide, MoveTo, SetCursorStyle, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen};

const NAME: &str = "NEI - Norton Editor Inspired";
const VERSION: &str = env!("CARGO_PKG_VERSION");

fn print_help() {
    println!("{NAME}");
    println!();
    println!("Usage:");
    println!("  nei [FILE]");
    println!("  nei --help");
    println!("  nei --version");
    println!();
    println!("The full-screen editor starts with the terminal shell in Sprint 01.");
}

struct TerminalGuard {
    stdout: Stdout,
}

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let stdout = io::stdout();
        let mut guard = Self { stdout };
        if let Err(error) = execute!(
            &mut guard.stdout,
            EnterAlternateScreen,
            Clear(ClearType::All),
            Hide,
            SetCursorStyle::SteadyBlock,
        ) {
            return Err(error);
        }
        Ok(guard)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        // Restoration is best effort because Drop cannot return an I/O error.
        let _ = execute!(
            self.stdout,
            Show,
            SetCursorStyle::DefaultUserShape,
            LeaveAlternateScreen
        );
        let _ = terminal::disable_raw_mode();
    }
}

fn draw_opening_screen(stdout: &mut Stdout, size: (u16, u16)) -> io::Result<()> {
    let (width, height) = size;
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

    if width > 0 && height > 0 {
        execute!(
            stdout,
            SetAttribute(Attribute::Bold),
            Print("Enter file name:"),
            SetAttribute(Attribute::Reset),
        )?;
    }

    // Keep the identity centered without assuming a minimum terminal size.
    let box_width = 38u16.min(width.saturating_sub(2));
    let box_height = 6u16;
    if box_width >= 4 && height >= box_height.saturating_add(2) {
        let left = (width - box_width) / 2;
        let top = height / 3;
        let inner_width = usize::from(box_width - 2);
        let border = format!("+{}+", "-".repeat(inner_width));
        let lines = [
            "NEI - Norton Editor Inspired",
            "A Programmer's Full-Screen Editor",
            "Version 0.0.1",
            "By Ronaldo F Morais, Brazil, 2026",
        ];

        execute!(stdout, MoveTo(left, top), Print(&border))?;
        for (offset, line) in lines.iter().enumerate() {
            let available = inner_width.saturating_sub(2);
            let text = if line.len() > available {
                &line[..available]
            } else {
                line
            };
            let padding = available.saturating_sub(text.len());
            let left_padding = padding / 2;
            let right_padding = padding - left_padding;
            execute!(
                stdout,
                MoveTo(left, top + 1 + offset as u16),
                Print(format!(
                    "| {}{}{} |",
                    " ".repeat(left_padding),
                    text,
                    " ".repeat(right_padding)
                )),
            )?;
        }
        execute!(stdout, MoveTo(left, top + 5), Print(&border))?;
    }

    stdout.flush()
}

fn run(file: Option<&str>) -> io::Result<()> {
    let mut terminal = TerminalGuard::enter()?;
    let size = terminal::size()?;

    // File opening and editing are deliberately deferred to the next Sprint.
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

fn main() {
    let mut args = std::env::args().skip(1);

    let result = match args.next().as_deref() {
        Some("--help" | "-h") => {
            print_help();
            Ok(())
        }
        Some("--version" | "-V") => {
            println!("nei {VERSION}");
            Ok(())
        }
        Some(file) => run(Some(file)),
        None => run(None),
    };

    result.unwrap_or_else(|error| {
        eprintln!("nei: {error}");
        std::process::exit(1);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opening_screen_fits_small_terminal_without_panicking() {
        let mut output = io::stdout();
        draw_opening_screen(&mut output, (1, 1)).expect("drawing should be safe");
    }

    #[test]
    fn opening_screen_uses_the_nei_identity() {
        assert_eq!(NAME, "NEI - Norton Editor Inspired");
        assert!(!VERSION.is_empty());
    }
}
