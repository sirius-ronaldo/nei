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

pub struct EditorWindow {
    pub document: Document,
    pub cursor: Position,
    pub viewport: Viewport,
    pub name: String,
}

impl EditorWindow {
    pub fn new(document: Document, name: impl Into<String>) -> Self {
        Self {
            document,
            cursor: Position::default(),
            viewport: Viewport::default(),
            name: name.into(),
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
}
