use std::io;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::document::Document;
use crate::editor_window::EditorWindow;
use crate::screen::{draw_editor, draw_opening_screen};
use crate::terminal::TerminalGuard;

pub fn run(file: Option<&str>) -> io::Result<()> {
    let mut terminal = TerminalGuard::enter()?;
    let mut editor = match file {
        Some(path) => EditorWindow::new(Document::from_path(std::path::Path::new(path))?, path),
        None => match open_from_prompt(&mut terminal) {
            Ok(editor) => editor,
            Err(error) if error.kind() == io::ErrorKind::Interrupted => return Ok(()),
            Err(error) => return Err(error),
        },
    };
    draw_editor(
        &mut terminal.stdout,
        &mut editor,
        crossterm::terminal::size()?,
    )?;

    loop {
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) if key.code == KeyCode::Esc => break,
                Event::Key(key) => {
                    handle_key(&mut editor, key.code, key.modifiers);
                    draw_editor(
                        &mut terminal.stdout,
                        &mut editor,
                        crossterm::terminal::size()?,
                    )?;
                }
                Event::Resize(width, height) => {
                    draw_editor(&mut terminal.stdout, &mut editor, (width, height))?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn open_from_prompt(terminal: &mut TerminalGuard) -> io::Result<EditorWindow> {
    let mut input = String::new();
    loop {
        let size = crossterm::terminal::size()?;
        draw_opening_screen(&mut terminal.stdout, size)?;
        crossterm::execute!(
            &mut terminal.stdout,
            crossterm::cursor::MoveTo(0, 0),
            crossterm::style::Print(format!("Enter file name: {}", input))
        )?;
        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Esc => {
                        return Err(io::Error::new(
                            io::ErrorKind::Interrupted,
                            "operation cancelled",
                        ));
                    }
                    KeyCode::Enter if !input.is_empty() => {
                        let document = Document::from_path(std::path::Path::new(&input))?;
                        return Ok(EditorWindow::new(document, input));
                    }
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Char(character)
                        if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
                    {
                        input.push(character)
                    }
                    _ => {}
                }
            }
        }
    }
}

fn handle_key(editor: &mut EditorWindow, code: KeyCode, modifiers: KeyModifiers) {
    let control = modifiers.contains(KeyModifiers::CONTROL);
    match (code, control) {
        (KeyCode::Left, true) => editor.word_left(),
        (KeyCode::Right, true) => editor.word_right(),
        (KeyCode::Left, false) => editor.move_left(),
        (KeyCode::Right, false) => editor.move_right(),
        (KeyCode::Up, _) => editor.move_up(),
        (KeyCode::Down, _) => editor.move_down(),
        (KeyCode::Home, true) => editor.file_start(),
        (KeyCode::End, true) => editor.file_end(),
        (KeyCode::Home, false) => editor.home(),
        (KeyCode::End, false) => editor.end(),
        (KeyCode::PageUp, _) => editor.page_up(),
        (KeyCode::PageDown, _) => editor.page_down(),
        _ => {}
    }
}
