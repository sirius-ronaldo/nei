use std::io;
use std::path::Path;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::document::{Document, TextMatch};
use crate::editor_window::EditorWindow;
use crate::screen::{draw_editor_layout, draw_opening_screen};
use crate::terminal::TerminalGuard;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InputMode {
    Editing,
    FileCommand,
    BlockCommand,
    Search,
    Replace,
    F5Command,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum CommandResult {
    Continue,
    Quit,
}

struct SearchState {
    needle: String,
    case_sensitive: bool,
}

struct ReplaceState {
    needle: String,
    replacement: String,
    case_sensitive: bool,
    current: Option<TextMatch>,
    replaced: usize,
    exhausted: bool,
}

pub fn run(file: Option<&str>) -> io::Result<()> {
    let mut terminal = TerminalGuard::enter()?;
    let mut editor = match file {
        Some(path) => {
            let document = if Path::new(path).is_file() {
                Document::from_path(Path::new(path))?
            } else {
                Document::empty()
            };
            EditorWindow::new(document, path)
        }
        None => match open_from_prompt(&mut terminal)? {
            Some(editor) => editor,
            None => return Ok(()),
        },
    };
    let mut other_editor = None;
    let mut active_window = 0usize;
    let mut mode = InputMode::Editing;
    let mut context = None;
    let mut last_search = String::new();
    let mut search = None;
    let mut replacement = None;
    draw_layout(
        &mut terminal.stdout,
        &mut editor,
        other_editor.as_mut(),
        active_window,
        crossterm::terminal::size()?,
        context.as_deref(),
    )?;

    loop {
        if event::poll(Duration::from_millis(250))? {
            match event::read()? {
                Event::Key(key) => {
                    let mut key = key;
                    if mode == InputMode::F5Command {
                        match key.code {
                            KeyCode::Char('f' | 'F') | KeyCode::Char('r' | 'R') => {
                                key.modifiers = KeyModifiers::CONTROL;
                                key.code = if matches!(key.code, KeyCode::Char('f' | 'F')) {
                                    KeyCode::Char('f')
                                } else {
                                    KeyCode::Char('h')
                                };
                                mode = InputMode::Editing;
                                context = None;
                            }
                            KeyCode::Char('w' | 'W') => {
                                let active = active_editor_mut(
                                    &mut editor,
                                    &mut other_editor,
                                    active_window,
                                );
                                if active.word_wrap {
                                    active.set_word_wrap(false, 0);
                                } else if let Some(width) =
                                    prompt_width(&mut terminal, "Word Wrap width: ")?
                                {
                                    active.set_word_wrap(true, width);
                                }
                                mode = InputMode::Editing;
                                context = None;
                                key.code = KeyCode::Null;
                            }
                            KeyCode::Esc => {
                                mode = InputMode::Editing;
                                context = None;
                                key.code = KeyCode::Null;
                            }
                            _ => {}
                        }
                    }
                    if mode == InputMode::Editing && key.code == KeyCode::F(5) {
                        mode = InputMode::F5Command;
                        context = Some("F5: Find   Replace   Word Wrap".to_owned());
                    }
                    if mode == InputMode::Editing && key.modifiers.contains(KeyModifiers::CONTROL) {
                        match key.code {
                            KeyCode::Char('f' | 'F') => {
                                if let Some(needle) =
                                    prompt_text(&mut terminal, "Find: ", &last_search)?
                                {
                                    last_search = needle.clone();
                                    if !needle.is_empty() {
                                        let active = active_editor_mut(
                                            &mut editor,
                                            &mut other_editor,
                                            active_window,
                                        );
                                        let state = SearchState {
                                            needle,
                                            case_sensitive: false,
                                        };
                                        if find_from_cursor(active, &state, true, false) {
                                            context = Some(search_context(&state));
                                            search = Some(state);
                                            mode = InputMode::Search;
                                        }
                                    }
                                }
                                draw_layout(
                                    &mut terminal.stdout,
                                    &mut editor,
                                    other_editor.as_mut(),
                                    active_window,
                                    crossterm::terminal::size()?,
                                    context.as_deref(),
                                )?;
                                continue;
                            }
                            KeyCode::Char('h' | 'H') => {
                                let Some(needle) =
                                    prompt_text(&mut terminal, "Find: ", &last_search)?
                                else {
                                    draw_layout(
                                        &mut terminal.stdout,
                                        &mut editor,
                                        other_editor.as_mut(),
                                        active_window,
                                        crossterm::terminal::size()?,
                                        context.as_deref(),
                                    )?;
                                    continue;
                                };
                                let Some(replacement_text) =
                                    prompt_text(&mut terminal, "Replace with: ", "")?
                                else {
                                    draw_layout(
                                        &mut terminal.stdout,
                                        &mut editor,
                                        other_editor.as_mut(),
                                        active_window,
                                        crossterm::terminal::size()?,
                                        context.as_deref(),
                                    )?;
                                    continue;
                                };
                                last_search = needle.clone();
                                if !needle.is_empty() {
                                    let active = active_editor_mut(
                                        &mut editor,
                                        &mut other_editor,
                                        active_window,
                                    );
                                    active.remove_block_markers();
                                    let state = ReplaceState {
                                        needle,
                                        replacement: replacement_text,
                                        case_sensitive: false,
                                        current: None,
                                        replaced: 0,
                                        exhausted: false,
                                    };
                                    let mut state = state;
                                    state.current = find_match(
                                        active,
                                        &state.needle,
                                        active.document.offset_of(active.cursor),
                                        true,
                                        state.case_sensitive,
                                    );
                                    if let Some(found) = state.current {
                                        active.cursor =
                                            active.document.position_at_offset(found.start);
                                        context = Some(replace_context(&state));
                                        replacement = Some(state);
                                        mode = InputMode::Replace;
                                    }
                                }
                                draw_layout(
                                    &mut terminal.stdout,
                                    &mut editor,
                                    other_editor.as_mut(),
                                    active_window,
                                    crossterm::terminal::size()?,
                                    context.as_deref(),
                                )?;
                                continue;
                            }
                            _ => {}
                        }
                    }
                    let result = match mode {
                        InputMode::Editing => {
                            let active =
                                active_editor_mut(&mut editor, &mut other_editor, active_window);
                            handle_editing_key(
                                active,
                                key.code,
                                key.modifiers,
                                &mut mode,
                                &mut context,
                            )
                        }
                        InputMode::FileCommand => {
                            if matches!(key.code, KeyCode::Char('x' | 'X')) {
                                exchange_windows(
                                    &mut terminal,
                                    &mut other_editor,
                                    &mut active_window,
                                )?;
                                mode = InputMode::Editing;
                                context = None;
                                CommandResult::Continue
                            } else {
                                let active = active_editor_mut(
                                    &mut editor,
                                    &mut other_editor,
                                    active_window,
                                );
                                handle_file_key(
                                    &mut terminal,
                                    active,
                                    key.code,
                                    &mut mode,
                                    &mut context,
                                )?
                            }
                        }
                        InputMode::BlockCommand => {
                            let source = if active_window == 0 {
                                other_editor.as_ref()
                            } else {
                                Some(&editor)
                            }
                            .cloned();
                            let active =
                                active_editor_mut(&mut editor, &mut other_editor, active_window);
                            handle_block_key(
                                active,
                                source.as_ref(),
                                key.code,
                                &mut mode,
                                &mut context,
                            )
                        }
                        InputMode::Search => {
                            let active =
                                active_editor_mut(&mut editor, &mut other_editor, active_window);
                            handle_search_key(
                                active,
                                key.code,
                                &mut mode,
                                &mut context,
                                &mut search,
                            )
                        }
                        InputMode::Replace => {
                            let active =
                                active_editor_mut(&mut editor, &mut other_editor, active_window);
                            handle_replace_key(
                                active,
                                key.code,
                                &mut mode,
                                &mut context,
                                &mut replacement,
                            )
                        }
                        InputMode::F5Command => CommandResult::Continue,
                    };
                    if result == CommandResult::Quit {
                        if other_editor.is_some() {
                            close_active_window(&mut editor, &mut other_editor, &mut active_window);
                            draw_layout(
                                &mut terminal.stdout,
                                &mut editor,
                                other_editor.as_mut(),
                                active_window,
                                crossterm::terminal::size()?,
                                context.as_deref(),
                            )?;
                            continue;
                        }
                        break;
                    }
                    draw_layout(
                        &mut terminal.stdout,
                        &mut editor,
                        other_editor.as_mut(),
                        active_window,
                        crossterm::terminal::size()?,
                        context.as_deref(),
                    )?;
                }
                Event::Resize(width, height) => {
                    draw_layout(
                        &mut terminal.stdout,
                        &mut editor,
                        other_editor.as_mut(),
                        active_window,
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

fn active_editor_mut<'a>(
    first: &'a mut EditorWindow,
    second: &'a mut Option<EditorWindow>,
    active: usize,
) -> &'a mut EditorWindow {
    if active == 0 {
        first
    } else {
        second.as_mut().expect("janela ativa deve existir")
    }
}

fn draw_layout(
    stdout: &mut std::io::Stdout,
    first: &mut EditorWindow,
    second: Option<&mut EditorWindow>,
    active: usize,
    size: (u16, u16),
    context: Option<&str>,
) -> io::Result<()> {
    draw_editor_layout(stdout, first, second, active, size, context)
}

fn handle_editing_key(
    editor: &mut EditorWindow,
    code: KeyCode,
    modifiers: KeyModifiers,
    mode: &mut InputMode,
    context: &mut Option<String>,
) -> CommandResult {
    if code == KeyCode::F(3) {
        *mode = InputMode::FileCommand;
        *context = Some(
            "F3 FILE: Exit-with-save   Quit   Save   eXchange-windows   New   Append   L   W"
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

fn find_match(
    editor: &EditorWindow,
    needle: &str,
    from: usize,
    forward: bool,
    case_sensitive: bool,
) -> Option<TextMatch> {
    editor
        .document
        .find_text(needle, from, forward, case_sensitive)
}

fn find_from_cursor(
    editor: &mut EditorWindow,
    state: &SearchState,
    forward: bool,
    advance: bool,
) -> bool {
    let mut offset = editor.document.offset_of(editor.cursor);
    if advance {
        offset = if forward {
            offset.saturating_add(1)
        } else {
            offset.saturating_sub(1)
        };
    }
    let Some(found) = find_match(editor, &state.needle, offset, forward, state.case_sensitive)
    else {
        return false;
    };
    editor.cursor = editor.document.position_at_offset(found.start);
    true
}

fn search_context(state: &SearchState) -> String {
    format!(
        "Find: {} [{}]  ↑/← Back  ↓/→ Forward  C Case  ESC Exit",
        state.needle,
        if state.case_sensitive { "C" } else { "I" }
    )
}

fn handle_search_key(
    editor: &mut EditorWindow,
    code: KeyCode,
    mode: &mut InputMode,
    context: &mut Option<String>,
    state: &mut Option<SearchState>,
) -> CommandResult {
    let Some(search) = state.as_mut() else {
        *mode = InputMode::Editing;
        return CommandResult::Continue;
    };
    let mut exit = false;
    match code {
        KeyCode::Esc => {
            *mode = InputMode::Editing;
            *context = None;
            exit = true;
        }
        KeyCode::Char('c' | 'C') => {
            search.case_sensitive = !search.case_sensitive;
            *context = Some(search_context(search));
        }
        KeyCode::Left | KeyCode::Up => {
            if !find_from_cursor(editor, search, false, true) {
                *mode = InputMode::Editing;
                *context = None;
                exit = true;
            }
        }
        KeyCode::Right | KeyCode::Down | KeyCode::Enter
            if !find_from_cursor(editor, search, true, true) =>
        {
            *mode = InputMode::Editing;
            *context = None;
            exit = true;
        }
        _ => {}
    }
    if exit {
        *state = None;
    }
    CommandResult::Continue
}

fn replace_context(state: &ReplaceState) -> String {
    if state.exhausted {
        return "No more occurrences   ESC Edit".to_owned();
    }
    format!(
        "Replace: {} -> {} [{}]  ENTER Replace  S Skip  A All  ESC Exit  ({})",
        state.needle,
        state.replacement,
        if state.case_sensitive { "C" } else { "I" },
        state.replaced
    )
}

fn next_replacement(editor: &mut EditorWindow, state: &mut ReplaceState, from: usize) {
    state.current = find_match(editor, &state.needle, from, true, state.case_sensitive);
    if let Some(found) = state.current {
        editor.cursor = editor.document.position_at_offset(found.start);
    }
}

fn handle_replace_key(
    editor: &mut EditorWindow,
    code: KeyCode,
    mode: &mut InputMode,
    context: &mut Option<String>,
    state: &mut Option<ReplaceState>,
) -> CommandResult {
    let Some(replace) = state.as_mut() else {
        *mode = InputMode::Editing;
        return CommandResult::Continue;
    };
    let mut exit = false;
    if replace.exhausted {
        if code == KeyCode::Esc {
            *mode = InputMode::Editing;
            *context = None;
            exit = true;
        }
        if exit {
            *state = None;
        }
        return CommandResult::Continue;
    }
    match code {
        KeyCode::Esc => {
            *mode = InputMode::Editing;
            *context = None;
            exit = true;
        }
        KeyCode::Char('c' | 'C') => {
            replace.case_sensitive = !replace.case_sensitive;
            *context = Some(replace_context(replace));
        }
        KeyCode::Char('s' | 'S') => {
            if let Some(found) = replace.current {
                next_replacement(editor, replace, found.start + found.length);
            }
            replace.exhausted = replace.current.is_none();
            *context = Some(replace_context(replace));
        }
        KeyCode::Enter => {
            if let Some(found) = replace.current {
                let start = found.start;
                let cursor = editor.document.replace_text(found, &replace.replacement);
                replace.replaced += 1;
                editor.cursor = cursor;
                next_replacement(editor, replace, start + replace.replacement.chars().count());
            }
            replace.exhausted = replace.current.is_none();
            *context = Some(replace_context(replace));
        }
        KeyCode::Char('a' | 'A') => {
            while let Some(found) = replace.current {
                let start = found.start;
                editor.document.replace_text(found, &replace.replacement);
                replace.replaced += 1;
                next_replacement(editor, replace, start + replace.replacement.chars().count());
            }
            replace.exhausted = true;
            *context = Some(replace_context(replace));
        }
        _ => {}
    }
    if exit {
        *state = None;
    }
    CommandResult::Continue
}

fn handle_block_key(
    editor: &mut EditorWindow,
    other_editor: Option<&EditorWindow>,
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
        'E' => editor.mark_to_line_end(),
        'F' => editor.find_next_marker(),
        'W' => {
            if let Some(other) = other_editor {
                editor.copy_block_from(other);
            }
        }
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
        'L' => {
            if let Some(path) = prompt_file_name(terminal, "Enter file name: ")?
                && let Ok(content) = std::fs::read_to_string(Path::new(&path))
            {
                editor.cursor = editor.document.insert_text(editor.cursor, &content);
            }
        }
        'W' => {
            let Some((start, end)) = editor.block.selection_range() else {
                return Ok(CommandResult::Continue);
            };
            let Some(path) = prompt_file_name(terminal, "Write block to file: ")? else {
                return Ok(CommandResult::Continue);
            };
            if Path::new(&path).is_file() && !confirm_overwrite(terminal, &path)? {
                return Ok(CommandResult::Continue);
            }
            if let Some(text) = editor.document.text_range(start, end) {
                std::fs::write(path, text)?;
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
        _ => {}
    }
    Ok(CommandResult::Continue)
}

fn exchange_windows(
    terminal: &mut TerminalGuard,
    second: &mut Option<EditorWindow>,
    active: &mut usize,
) -> io::Result<()> {
    if second.is_none() {
        if let Some(path) = prompt_file_name(terminal, "Enter file name: ")? {
            let document = if Path::new(&path).is_file() {
                Document::from_path(Path::new(&path))?
            } else {
                Document::empty()
            };
            *second = Some(EditorWindow::new(document, path));
            *active = 1;
        }
    } else {
        *active = 1 - *active;
    }
    Ok(())
}

fn close_active_window(
    first: &mut EditorWindow,
    second: &mut Option<EditorWindow>,
    active: &mut usize,
) {
    if *active == 0 {
        *first = second.take().expect("a segunda janela deve existir");
    } else {
        *second = None;
        *active = 0;
    }
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
    loop {
        let Some(path) = prompt_file_name(terminal, "Enter file name: ")? else {
            return Ok(None);
        };
        if allow_missing || Path::new(&path).is_file() {
            return Ok(Some(path));
        }
    }
}

fn prompt_file_name(terminal: &mut TerminalGuard, label: &str) -> io::Result<Option<String>> {
    let mut input = String::new();
    loop {
        execute_prompt(terminal, &format!("{label}{input}"))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter if !input.is_empty() => {
                    return Ok(Some(input));
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

fn prompt_text(
    terminal: &mut TerminalGuard,
    label: &str,
    initial: &str,
) -> io::Result<Option<String>> {
    let mut input = initial.to_owned();
    loop {
        execute_prompt(terminal, &format!("{label}{input}"))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(None),
                KeyCode::Enter => return Ok(Some(input)),
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

fn prompt_width(terminal: &mut TerminalGuard, label: &str) -> io::Result<Option<usize>> {
    loop {
        let Some(input) = prompt_text(terminal, label, "")? else {
            return Ok(None);
        };
        if let Ok(width) = input.trim().parse::<usize>()
            && width > 0
        {
            return Ok(Some(width));
        }
    }
}

fn confirm_overwrite(terminal: &mut TerminalGuard, path: &str) -> io::Result<bool> {
    execute_prompt(terminal, &format!("Replace {path}? (Y/N)"))?;
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

fn execute_prompt(terminal: &mut TerminalGuard, text: &str) -> io::Result<()> {
    let size = crossterm::terminal::size()?;
    draw_opening_screen(&mut terminal.stdout, size)?;
    crossterm::execute!(
        &mut terminal.stdout,
        crossterm::cursor::MoveTo(0, 0),
        crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine),
        crossterm::style::Print(text)
    )
}

fn open_from_prompt(terminal: &mut TerminalGuard) -> io::Result<Option<EditorWindow>> {
    let Some(path) = prompt_file_name(terminal, "Enter file name: ")? else {
        return Ok(None);
    };
    let document = if Path::new(&path).is_file() {
        Document::from_path(Path::new(&path))?
    } else {
        Document::empty()
    };
    Ok(Some(EditorWindow::new(document, path)))
}
