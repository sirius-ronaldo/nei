use std::io::{self, Stdout, Write};

use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::style::{Attribute, Print, SetAttribute};
use crossterm::terminal::{Clear, ClearType};

const IDENTITY_LINES: [&str; 4] = [
    "NEI - Norton Editor Inspired",
    "A Programmer's Full-Screen Editor",
    "Version 0.0.1",
    "By Ronaldo F Morais, Brazil, 2026",
];

pub fn draw_opening_screen(stdout: &mut Stdout, size: (u16, u16)) -> io::Result<()> {
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

    // Mantém a identidade centralizada sem assumir um tamanho mínimo de terminal.
    let box_width = 38u16.min(width.saturating_sub(2));
    let box_height = 6u16;
    if box_width >= 4 && height >= box_height.saturating_add(2) {
        let left = (width - box_width) / 2;
        let top = height / 3;
        let inner_width = usize::from(box_width - 2);
        let border = format!("+{}+", "-".repeat(inner_width));

        execute!(stdout, MoveTo(left, top), Print(&border))?;
        for (offset, line) in IDENTITY_LINES.iter().enumerate() {
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
        assert_eq!(IDENTITY_LINES[0], "NEI - Norton Editor Inspired");
        assert!(IDENTITY_LINES[2].starts_with("Version "));
    }
}
