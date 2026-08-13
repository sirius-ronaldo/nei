use std::io::{self, Stdout, Write};

use crossterm::cursor::{MoveTo, Show};
use crossterm::execute;
use crossterm::style::{
    Attribute, Color, Print, ResetColor, SetAttribute, SetBackgroundColor, SetForegroundColor,
};
use crossterm::terminal::{Clear, ClearType};

const IDENTITY_LINES: [&str; 4] = [
    "NEI - Norton Editor Inspired",
    "A Programmer's Full-Screen Editor",
    "Version 0.0.1",
    "By Ronaldo F Morais, Brazil, 2026",
];

const HELP_ROWS: [(&str, &str); 22] = [
    ("NEI - HELP", "F1-Help"),
    ("", ""),
    ("CURSOR CONTROL", "FILE COMMANDS"),
    ("← → ↑ ↓        Move cursor", "F3 E    Save and exit"),
    ("Ctrl+←/Ctrl+→  Move by word", "F3 S    Save"),
    (
        "Home/End       Line beginning/end",
        "F3 Q    Quit without save",
    ),
    ("PgUp/PgDn      Page up/down", "F3 N    Edit new file"),
    (
        "Ctrl+Home/End  File beginning/end",
        "F3 X    Exchange windows",
    ),
    ("", "F3 W    Write marked block"),
    ("DELETE COMMANDS", "F3 L    Load file at cursor"),
    ("Backspace      Delete left", "F3 A    Append file"),
    ("Delete         Delete right", ""),
    ("Ctrl+W         Delete word left", "BLOCK COMMANDS"),
    ("Alt+W          Delete word right", "F4 S    Set marker"),
    (
        "Ctrl+L         Delete to line begin",
        "F4 R    Remove markers",
    ),
    ("Alt+L          Delete to line end", "F4 D    Delete block"),
    ("Alt+K          Kill line", "F4 C    Copy block"),
    ("Ctrl+U         Undelete", "F4 W    Copy from window"),
    ("", "F4 M    Move block"),
    (
        "SEARCH / REPLACE / WRAP",
        "F4 L/E/F Mark line/end/find marker",
    ),
    (
        "(Ctrl+F or F5 F  Find)  (Ctrl+H or F5 R  Replace)   (F5 W  Word Wrap)",
        "",
    ),
    ("Press any key to return to the editor.", ""),
];

pub fn draw_help(stdout: &mut Stdout, size: (u16, u16)) -> io::Result<()> {
    let (width, height) = size;
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
    for (row, (left, right)) in HELP_ROWS.iter().enumerate() {
        if row >= usize::from(height) {
            break;
        }
        let line = if row == 0 {
            format!("{left:<59}{right}")
        } else if right.is_empty() {
            (*left).to_owned()
        } else {
            format!("{left:<41}{right}")
        };
        let text: String = line.chars().take(usize::from(width)).collect();
        execute!(stdout, MoveTo(0, row as u16), Print(text))?;
    }
    stdout.flush()
}

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

use crate::editor_window::EditorWindow;

pub fn draw_editor_with_context(
    stdout: &mut Stdout,
    window: &mut EditorWindow,
    size: (u16, u16),
    context: Option<&str>,
) -> io::Result<()> {
    let (width, height) = size;
    let text_height = usize::from(height.saturating_sub(1));
    let text_width = usize::from(width.saturating_sub(1));
    window.update_viewport(text_width, text_height);
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;

    for row in 0..text_height {
        let line_index = window.viewport.top_line + row;
        if line_index >= window.document.line_count() {
            break;
        }
        if width == 0 {
            continue;
        }
        if let Some(marker) = window.block.marker_at(line_index) {
            execute!(
                stdout,
                MoveTo(0, row as u16),
                SetBackgroundColor(Color::White),
                SetForegroundColor(Color::Black),
                Print(marker),
                ResetColor
            )?;
        }
        let line = window.document.line(line_index);
        for (visible_column, character) in line
            .chars()
            .skip(window.viewport.left_column)
            .take(text_width)
            .enumerate()
        {
            let column = window.viewport.left_column + visible_column;
            let position = crate::document::Position {
                line: line_index,
                column,
            };
            execute!(
                stdout,
                MoveTo((visible_column + 1) as u16, row as u16),
                SetAttribute(if window.block.contains(position) {
                    Attribute::Bold
                } else {
                    Attribute::NormalIntensity
                }),
                Print(character),
                SetAttribute(Attribute::Reset)
            )?;
        }
    }

    if height > 0 {
        let status = context.map(str::to_owned).unwrap_or_else(|| {
            format!(
                "Line={}    Col={}    F1-Help             {}             {}    WW={}",
                window.cursor.line + 1,
                window.cursor.column + 1,
                window.name,
                if window.insert_mode {
                    "Insert"
                } else {
                    "Overwrite"
                },
                if window.word_wrap { "On" } else { "Off" }
            )
        });
        let status: String = status.chars().take(usize::from(width)).collect();
        draw_status(stdout, 0, height - 1, width, &status)?;
    }

    if width > 0 && text_height > 0 {
        let x = window
            .cursor
            .column
            .saturating_sub(window.viewport.left_column)
            .min(text_width.saturating_sub(1));
        let y = window
            .cursor
            .line
            .saturating_sub(window.viewport.top_line)
            .min(text_height - 1);
        execute!(
            stdout,
            Show,
            MoveTo((x + 1).min(usize::from(width - 1)) as u16, y as u16)
        )?;
    }
    stdout.flush()
}

pub fn draw_editor_layout(
    stdout: &mut Stdout,
    first: &mut EditorWindow,
    second: Option<&mut EditorWindow>,
    active: usize,
    size: (u16, u16),
    context: Option<&str>,
) -> io::Result<()> {
    let (width, height) = size;
    execute!(stdout, MoveTo(0, 0), Clear(ClearType::All))?;
    let Some(second) = second else {
        return draw_editor_with_context(stdout, first, size, context);
    };

    let first_height = usize::from(height / 2).max(1);
    let second_height = usize::from(height).saturating_sub(first_height).max(1);
    let status = if active == 0 {
        status_text(first, context)
    } else {
        status_text(second, context)
    };
    draw_pane(stdout, first, width, first_height as u16, 0, true)?;
    draw_pane(
        stdout,
        second,
        width,
        second_height as u16,
        first_height as u16,
        false,
    )?;
    draw_status(stdout, 0, first_height as u16 - 1, width, &status)?;
    draw_active_cursor(
        stdout,
        if active == 0 { first } else { second },
        active,
        width,
        first_height as u16,
    )?;
    stdout.flush()
}

fn draw_pane(
    stdout: &mut Stdout,
    window: &mut EditorWindow,
    width: u16,
    pane_height: u16,
    top: u16,
    reserve_status: bool,
) -> io::Result<()> {
    let text_height = if reserve_status {
        usize::from(pane_height.saturating_sub(1))
    } else {
        usize::from(pane_height)
    };
    let text_width = usize::from(width.saturating_sub(1));
    window.update_viewport(text_width, text_height);
    for row in 0..text_height {
        let line_index = window.viewport.top_line + row;
        if line_index >= window.document.line_count() || width == 0 {
            break;
        }
        if let Some(marker) = window.block.marker_at(line_index) {
            execute!(
                stdout,
                MoveTo(0, top + row as u16),
                SetBackgroundColor(Color::White),
                SetForegroundColor(Color::Black),
                Print(marker),
                ResetColor
            )?;
        }
        for (visible_column, character) in window
            .document
            .line(line_index)
            .chars()
            .skip(window.viewport.left_column)
            .take(text_width)
            .enumerate()
        {
            let position = crate::document::Position {
                line: line_index,
                column: window.viewport.left_column + visible_column,
            };
            execute!(
                stdout,
                MoveTo((visible_column + 1) as u16, top + row as u16),
                SetAttribute(if window.block.contains(position) {
                    Attribute::Bold
                } else {
                    Attribute::NormalIntensity
                }),
                Print(character),
                SetAttribute(Attribute::Reset)
            )?;
        }
    }
    Ok(())
}

fn status_text(window: &EditorWindow, context: Option<&str>) -> String {
    context.map(str::to_owned).unwrap_or_else(|| {
        format!(
            "Line={}    Col={}    F1-Help             {}             {}    WW={}",
            window.cursor.line + 1,
            window.cursor.column + 1,
            window.name,
            if window.insert_mode {
                "Insert"
            } else {
                "Overwrite"
            },
            if window.word_wrap { "On" } else { "Off" }
        )
    })
}

fn draw_active_cursor(
    stdout: &mut Stdout,
    window: &mut EditorWindow,
    active: usize,
    width: u16,
    separator: u16,
) -> io::Result<()> {
    if width == 0 {
        return Ok(());
    }
    let pane_top = if active == 0 { 0 } else { separator };
    let pane_height = if active == 0 {
        separator.saturating_sub(1)
    } else {
        u16::MAX.saturating_sub(separator)
    };
    let text_height = usize::from(pane_height);
    if text_height == 0 {
        return Ok(());
    }
    let x = window
        .cursor
        .column
        .saturating_sub(window.viewport.left_column)
        .min(usize::from(width.saturating_sub(2)));
    let y = window
        .cursor
        .line
        .saturating_sub(window.viewport.top_line)
        .min(text_height - 1);
    execute!(stdout, Show, MoveTo((x + 1) as u16, pane_top + y as u16))
}

fn draw_status(stdout: &mut Stdout, x: u16, y: u16, width: u16, text: &str) -> io::Result<()> {
    let width = usize::from(width);
    let content: String = text.chars().take(width).collect();
    let line = format!("{content:<width$}");
    execute!(
        stdout,
        MoveTo(x, y),
        SetBackgroundColor(Color::White),
        SetForegroundColor(Color::Black),
        Print(line),
        ResetColor
    )
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

    #[test]
    fn help_screen_fits_reference_terminal_and_small_terminals() {
        let mut output = io::stdout();
        draw_help(&mut output, (80, 24)).expect("help should fit reference terminal");
        draw_help(&mut output, (1, 1)).expect("help should tolerate small terminals");
    }
}
