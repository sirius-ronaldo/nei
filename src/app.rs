use std::io;
use std::path::Path;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::document::Document;
use crate::editor_window::EditorWindow;
use crate::screen::{draw_editor_with_context, draw_opening_screen};
use crate::terminal::TerminalGuard;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InputMode {
    Editing,
    FileCommand,
    BlockCommand,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandResult {
    Continue,
    Quit,
}

pub fn run(file: Option<&str>) -> io::Result<()> {
    let mut terminal = TerminalGuard::enter()?;
    let mut editor = match file {
        Some(path) => EditorWindow::new(Document::from_path(Path::new(path))?, path),
        None => match open_from_prompt(&mut terminal)? {
            Some(editor) => editor,
            None => return Ok(()),
        },
    };
    let mut mode = InputMode::Editing;
    let mut context = None;
    draw_editor_with_context(
        &mut terminal.stdout,
        &mut editor,
        crossterm::terminal::size()?,
        context.as_deref(),
    )?;

    loop {
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) => {
                    let result = match mode {
                        InputMode::Editing => handle_editing_key(
                            &mut editor,
                            key.code,
                            key.modifiers,
                            &mut mode,
                            &mut context,
                        ),
                        InputMode::FileCommand => handle_file_key(
                            &mut terminal,
                            &mut editor,
                            key.code,
                            &mut mode,
                            &mut context,
                        )?,
                        InputMode::BlockCommand => {
                            handle_block_key(&mut editor, key.code, &mut mode, &mut context)
                        }
                    };
                    if result == CommandResult::Quit {
                        break;
                    }
                    draw_editor_with_context(
                        &mut terminal.stdout,
                        &mut editor,
                        crossterm::terminal::size()?,
                        context.as_deref(),
                    )?;
                }
                Event::Resize(width, height) => {
                    draw_editor_with_context(
                        &mut terminal.stdout,
                        &mut editor,
                        (width, height),
                        context.as_deref(),
                    )?;
                }
                _ => {}
            }
        }
    }

    Ok(())
}

fn handle_editing_key(
    editor: &mut EditorWindow,
    code: KeyCode,
    modifiers: KeyModifiers,
    mode: &mut InputMode,
    context: &mut Option<String>,
) -> CommandResult {
    if code == KeyCode::Esc {
        return CommandResult::Quit;
    }
    if code == KeyCode::F(3) {
        *mode = InputMode::FileCommand;
        *context = Some(
            "F3 FILE: Exit-with-save   Quit   Save   eXchange-windows   New   Append   L   W   C"
                .to_owned(),
        );
        return CommandResult::Continue;
    }
    if code == KeyCode::F(4) {
        *mode = InputMode::BlockCommand;
        *context = Some(
            "F4 BLOCK: Set-marker   Copy   Move   Delete-block   Remove-marker   W   L   E   F"
                .to_owned(),
        );
        return CommandResult::Continue;
    }

    let control = modifiers.contains(KeyModifiers::CONTROL);
    let alt = modifiers.contains(KeyModifiers::ALT);
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
        (KeyCode::Insert, false) => editor.toggle_insert_mode(),
        (KeyCode::Enter, false) => editor.new_line(),
        (KeyCode::Backspace, false) => editor.backspace(),
        (KeyCode::Delete, false) => editor.delete(),
        (KeyCode::Char('w'), true) => editor.delete_word_left(),
        (KeyCode::Char('l'), true) => editor.delete_to_line_beginning(),
        (KeyCode::Char('u'), true) => editor.undelete(),
        (KeyCode::Char('w'), false) if alt => editor.delete_word_right(),
        (KeyCode::Char('l'), false) if alt => editor.delete_to_line_end(),
        (KeyCode::Char('k'), false) if alt => editor.kill_line(),
        (KeyCode::Char(character), false) if !alt => editor.insert_char(character),
        _ => {}
    }
    CommandResult::Continue
}

fn handle_block_key(
    editor: &mut EditorWindow,
    code: KeyCode,
    mode: &mut InputMode,
    context: &mut Option<String>,
) -> CommandResult {
    if code == KeyCode::Esc {
        *mode = InputMode::Editing;
        *context = None;
        return CommandResult::Continue;
    }
    let KeyCode::Char(command) = code else {
        return CommandResult::Continue;
    };
    *mode = InputMode::Editing;
    *context = None;
    match command.to_ascii_uppercase() {
        'S' => editor.set_block_marker(),
        'R' => editor.remove_block_markers(),
        'C' => editor.copy_block(),
        'M' => editor.move_block(),
        'D' => editor.delete_block(),
        'L' => editor.mark_line(),
        'F' => editor.find_next_marker(),
        _ => {}
    }
    CommandResult::Continue
}

fn handle_file_key(
    terminal: &mut TerminalGuard,
    editor: &mut EditorWindow,
    code: KeyCode,
    mode: &mut InputMode,
    context: &mut Option<String>,
) -> io::Result<CommandResult> {
    if code == KeyCode::Esc {
        *mode = InputMode::Editing;
        *context = None;
        return Ok(CommandResult::Continue);
    }
    let KeyCode::Char(command) = code else {
        return Ok(CommandResult::Continue);
    };
    *mode = InputMode::Editing;
    *context = None;
    match command.to_ascii_uppercase() {
        'E' => {
            editor.document.save_to_path(Path::new(&editor.name))?;
            return Ok(CommandResult::Quit);
        }
        'S' => editor.document.save_to_path(Path::new(&editor.name))?,
        'Q' => {
            if editor.document.modified && !confirm_quit(terminal)? {
                return Ok(CommandResult::Continue);
            }
            return Ok(CommandResult::Quit);
        }
        'A' => {
            if let Some(path) = prompt_file(terminal, false)? {
                let content = Document::from_path(Path::new(&path))?.as_text();
                let end = editor.document.line_count().saturating_sub(1);
                let position = crate::document::Position {
                    line: end,
                    column: editor.document.line_length(end),
                };
                editor.document.insert_text(position, &content);
            }
        }
        'N' => {
            // A confirmação explícita evita descartar alterações silenciosamente.
            if editor.document.modified && !confirm_new_file(terminal, editor)? {
                return Ok(CommandResult::Continue);
            }
            if let Some(path) = prompt_file(terminal, true)? {
                let document = if Path::new(&path).exists() {
                    Document::from_path(Path::new(&path))?
                } else {
                    Document::empty()
                };
                *editor = EditorWindow::new(document, path);
            }
        }
        // F3 X/L/W/C são reservados às sprints que definem suas semânticas.
        _ => {}
    }
    Ok(CommandResult::Continue)
}

fn confirm_new_file(terminal: &mut TerminalGuard, editor: &mut EditorWindow) -> io::Result<bool> {
    execute_prompt(terminal, "Save changes before opening a new file? (Y/N)")?;
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('n' | 'N') => return Ok(true),
                KeyCode::Char('y' | 'Y') => {
                    editor.document.save_to_path(Path::new(&editor.name))?;
                    return Ok(true);
                }
                KeyCode::Esc => return Ok(false),
                _ => {}
            }
        }
    }
}

fn confirm_quit(terminal: &mut TerminalGuard) -> io::Result<bool> {
    execute_prompt(terminal, "Quit without saving? (Y/N)")?;
    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('y' | 'Y') => return Ok(true),
                KeyCode::Char('n' | 'N') | KeyCode::Esc => return Ok(false),
                _ => {}
            }
        }
    }
}

fn prompt_file(terminal: &mut TerminalGuard, allow_missing: bool) -> io::Result<Option<String>> {
    let mut input = String::new();
    loop {
        execute_prompt(terminal, &format!("Enter file name: {input}"))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter if !input.is_empty() => {
                    if allow_missing || Path::new(&input).is_file() {
                        return Ok(Some(input));
                    }
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Char(character)
                    if key.modifiers.is_empty() || key.modifiers == KeyModifiers::SHIFT =>
                {
                    input.push(character);
                }
                _ => {}
            }
        }
    }
}

fn execute_prompt(terminal: &mut TerminalGuard, text: &str) -> io::Result<()> {
    let size = crossterm::terminal::size()?;
    draw_opening_screen(&mut terminal.stdout, size)?;
    crossterm::execute!(
        &mut terminal.stdout,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::style::Print(text)
    )
}

fn open_from_prompt(terminal: &mut TerminalGuard) -> io::Result<Option<EditorWindow>> {
    let Some(path) = prompt_file(terminal, false)? else {
        return Ok(None);
    };
    Ok(Some(EditorWindow::new(
        Document::from_path(Path::new(&path))?,
        path,
    )))
}
