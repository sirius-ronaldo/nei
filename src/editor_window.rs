use crate::document::{Document, Position};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Viewport {
    pub top_line: usize,
    pub left_column: usize,
    pub height: usize,
    pub width: usize,
}

impl Viewport {
    pub fn keep_visible(&mut self, cursor: Position) {
        if cursor.line < self.top_line {
            self.top_line = cursor.line;
        } else if self.height > 0 && cursor.line >= self.top_line + self.height {
            self.top_line = cursor.line + 1 - self.height;
        }
        if cursor.column < self.left_column {
            self.left_column = cursor.column;
        } else if self.width > 0 && cursor.column >= self.left_column + self.width {
            self.left_column = cursor.column + 1 - self.width;
        }
    }
}

#[derive(Clone)]
pub struct EditorWindow {
    pub document: Document,
    pub cursor: Position,
    pub viewport: Viewport,
    pub name: String,
    pub insert_mode: bool,
    last_deletion: Option<DeletedText>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeletedText {
    position: Position,
    text: String,
}

impl EditorWindow {
    pub fn new(document: Document, name: impl Into<String>) -> Self {
        Self {
            document,
            cursor: Position::default(),
            viewport: Viewport::default(),
            name: name.into(),
            insert_mode: true,
            last_deletion: None,
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor.column > 0 {
            self.cursor.column -= 1;
        } else if self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = self.document.line_length(self.cursor.line);
        }
    }

    pub fn move_right(&mut self) {
        let length = self.document.line_length(self.cursor.line);
        if self.cursor.column < length {
            self.cursor.column += 1;
        } else if self.cursor.line + 1 < self.document.line_count() {
            self.cursor.line += 1;
            self.cursor.column = 0;
        }
    }

    pub fn move_up(&mut self) {
        if self.cursor.line > 0 {
            self.cursor.line -= 1;
        }
        self.cursor.column = self
            .cursor
            .column
            .min(self.document.line_length(self.cursor.line));
    }

    pub fn move_down(&mut self) {
        if self.cursor.line + 1 < self.document.line_count() {
            self.cursor.line += 1;
        }
        self.cursor.column = self
            .cursor
            .column
            .min(self.document.line_length(self.cursor.line));
    }

    pub fn home(&mut self) {
        self.cursor.column = 0;
    }
    pub fn end(&mut self) {
        self.cursor.column = self.document.line_length(self.cursor.line);
    }
    pub fn file_start(&mut self) {
        self.cursor = Position::default();
    }
    pub fn file_end(&mut self) {
        self.cursor.line = self.document.line_count().saturating_sub(1);
        self.end();
    }

    pub fn word_left(&mut self) {
        self.home_if_empty_or_previous_line();
        let chars: Vec<char> = self.document.line(self.cursor.line).chars().collect();
        while self.cursor.column > 0 && chars[self.cursor.column - 1].is_whitespace() {
            self.cursor.column -= 1;
        }
        while self.cursor.column > 0 && !chars[self.cursor.column - 1].is_whitespace() {
            self.cursor.column -= 1;
        }
    }

    pub fn word_right(&mut self) {
        let chars: Vec<char> = self.document.line(self.cursor.line).chars().collect();
        while self.cursor.column < chars.len() && !chars[self.cursor.column].is_whitespace() {
            self.cursor.column += 1;
        }
        while self.cursor.column < chars.len() && chars[self.cursor.column].is_whitespace() {
            self.cursor.column += 1;
        }
    }

    pub fn toggle_insert_mode(&mut self) {
        self.insert_mode = !self.insert_mode;
    }

    pub fn insert_char(&mut self, character: char) {
        self.cursor = if self.insert_mode {
            self.document
                .insert_text(self.cursor, &character.to_string())
        } else {
            self.document.replace_char(self.cursor, character)
        };
    }

    pub fn new_line(&mut self) {
        self.cursor = self.document.insert_text(self.cursor, "\n");
    }

    pub fn backspace(&mut self) {
        if self.cursor.column > 0 {
            let start = Position {
                line: self.cursor.line,
                column: self.cursor.column - 1,
            };
            self.delete_range(start, self.cursor);
        } else if self.cursor.line > 0 {
            let start = Position {
                line: self.cursor.line - 1,
                column: self.document.line_length(self.cursor.line - 1),
            };
            self.delete_range(start, self.cursor);
        }
    }

    pub fn delete(&mut self) {
        let end = self.next_position(self.cursor);
        self.delete_range(self.cursor, end);
    }

    pub fn delete_word_left(&mut self) {
        let mut start = self.clone();
        start.word_left();
        self.delete_range(start.cursor, self.cursor);
    }

    pub fn delete_word_right(&mut self) {
        let mut end = self.clone();
        end.word_right();
        self.delete_range(self.cursor, end.cursor);
    }

    pub fn delete_to_line_beginning(&mut self) {
        self.delete_range(
            Position {
                line: self.cursor.line,
                column: 0,
            },
            self.cursor,
        );
    }

    pub fn delete_to_line_end(&mut self) {
        self.delete_range(
            self.cursor,
            Position {
                line: self.cursor.line,
                column: self.document.line_length(self.cursor.line),
            },
        );
    }

    pub fn kill_line(&mut self) {
        let line = self.cursor.line;
        let start = if line + 1 < self.document.line_count() {
            Position { line, column: 0 }
        } else if line > 0 {
            Position {
                line: line - 1,
                column: self.document.line_length(line - 1),
            }
        } else {
            Position { line, column: 0 }
        };
        let end = if line + 1 < self.document.line_count() {
            Position {
                line: line + 1,
                column: 0,
            }
        } else {
            Position {
                line,
                column: self.document.line_length(line),
            }
        };
        self.delete_range(start, end);
    }

    pub fn undelete(&mut self) {
        if let Some(deleted) = self.last_deletion.take() {
            self.cursor = self.document.insert_text(deleted.position, &deleted.text);
            self.cursor = deleted.position;
        }
    }

    fn delete_range(&mut self, start: Position, end: Position) {
        if let Some(text) = self.document.delete_range(start, end) {
            self.last_deletion = Some(DeletedText {
                position: start,
                text,
            });
            self.cursor = self.document.clamp(start);
        }
    }

    fn next_position(&self, position: Position) -> Position {
        if position.column < self.document.line_length(position.line) {
            Position {
                line: position.line,
                column: position.column + 1,
            }
        } else if position.line + 1 < self.document.line_count() {
            Position {
                line: position.line + 1,
                column: 0,
            }
        } else {
            position
        }
    }

    fn home_if_empty_or_previous_line(&mut self) {
        if self.cursor.column == 0 && self.cursor.line > 0 {
            self.cursor.line -= 1;
            self.cursor.column = self.document.line_length(self.cursor.line);
        }
    }

    pub fn page_up(&mut self) {
        self.cursor.line = self.cursor.line.saturating_sub(self.viewport.height.max(1));
        self.cursor.column = self
            .cursor
            .column
            .min(self.document.line_length(self.cursor.line));
    }
    pub fn page_down(&mut self) {
        self.cursor.line = (self.cursor.line + self.viewport.height.max(1))
            .min(self.document.line_count().saturating_sub(1));
        self.cursor.column = self
            .cursor
            .column
            .min(self.document.line_length(self.cursor.line));
    }
    pub fn update_viewport(&mut self, width: usize, height: usize) {
        self.cursor = self.document.clamp(self.cursor);
        self.viewport.width = width;
        self.viewport.height = height;
        self.viewport.keep_visible(self.cursor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn window() -> EditorWindow {
        EditorWindow::new(Document::from_text("um dois\nlinha"), "teste")
    }

    #[test]
    fn cursor_stays_within_document() {
        let mut editor = window();
        editor.move_left();
        editor.move_up();
        editor.file_end();
        editor.move_right();
        assert_eq!(editor.cursor, Position { line: 1, column: 5 });
        editor.file_start();
        editor.move_left();
        assert_eq!(editor.cursor, Position::default());
    }

    #[test]
    fn navigation_commands_work() {
        let mut editor = window();
        editor.word_right();
        assert_eq!(editor.cursor.column, 3);
        editor.word_right();
        assert_eq!(editor.cursor.column, 7);
        editor.word_left();
        assert_eq!(editor.cursor.column, 3);
        editor.end();
        editor.page_up();
        editor.page_down();
        assert_eq!(editor.cursor.line, 1);
    }

    #[test]
    fn insert_delete_and_single_undelete_work() {
        let mut editor = EditorWindow::new(Document::from_text("abc"), "teste");
        editor.cursor.column = 1;
        editor.insert_char('X');
        assert_eq!(editor.document.line(0), "aXbc");
        editor.delete_word_left();
        assert_eq!(editor.document.line(0), "bc");
        editor.insert_char('Z');
        editor.undelete();
        assert_eq!(editor.document.line(0), "aXZbc");
        editor.undelete();
        assert_eq!(editor.document.line(0), "aXZbc");
    }

    #[test]
    fn enter_and_backspace_join_lines() {
        let mut editor = EditorWindow::new(Document::from_text("ab"), "teste");
        editor.cursor.column = 1;
        editor.new_line();
        assert_eq!(editor.document.line_count(), 2);
        editor.backspace();
        assert_eq!(editor.document.line(0), "ab");
        assert_eq!(editor.document.line_count(), 1);
    }

    #[test]
    fn kill_line_removes_the_entire_line_and_can_be_undeleted() {
        let mut editor =
            EditorWindow::new(Document::from_text("primeira\nsegunda\nterceira"), "teste");
        editor.cursor = Position { line: 1, column: 3 };
        editor.kill_line();
        assert_eq!(editor.document.line_count(), 2);
        assert_eq!(editor.document.line(0), "primeira");
        assert_eq!(editor.document.line(1), "terceira");
        editor.undelete();
        assert_eq!(editor.document.line_count(), 3);
        assert_eq!(editor.document.line(1), "segunda");

        editor.file_end();
        editor.kill_line();
        assert_eq!(editor.document.line_count(), 2);
        assert_eq!(editor.document.line(1), "segunda");
    }

    #[test]
    fn kill_only_line_leaves_an_empty_document_line() {
        let mut editor = EditorWindow::new(Document::from_text("conteúdo"), "teste");
        editor.kill_line();
        assert_eq!(editor.document.line_count(), 1);
        assert_eq!(editor.document.line(0), "");
    }
}
