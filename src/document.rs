use std::io;
use std::path::Path;

/// Posição lógica no documento. A coluna é contada em caracteres Unicode.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Document {
    lines: Vec<String>,
}

impl Document {
    pub fn empty() -> Self {
        Self {
            lines: vec![String::new()],
        }
    }

    pub fn from_text(text: &str) -> Self {
        if text.is_empty() {
            return Self::empty();
        }
        let mut lines: Vec<String> = text.split('\n').map(str::to_owned).collect();
        if lines.is_empty() {
            lines.push(String::new());
        }
        for line in &mut lines {
            if line.ends_with('\r') {
                line.pop();
            }
        }
        Self { lines }
    }

    pub fn from_path(path: &Path) -> io::Result<Self> {
        Ok(Self::from_text(&std::fs::read_to_string(path)?))
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn line(&self, index: usize) -> &str {
        self.lines.get(index).map(String::as_str).unwrap_or("")
    }

    pub fn line_length(&self, index: usize) -> usize {
        self.line(index).chars().count()
    }

    pub fn clamp(&self, position: Position) -> Position {
        let line = position.line.min(self.line_count().saturating_sub(1));
        Position {
            line,
            column: position.column.min(self.line_length(line)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_and_trailing_newline_are_valid_documents() {
        assert_eq!(Document::from_text("").line_count(), 1);
        assert_eq!(Document::from_text("a\n").line_count(), 2);
    }

    #[test]
    fn utf8_columns_are_character_based() {
        let document = Document::from_text("ação");
        assert_eq!(document.line_length(0), 4);
        assert_eq!(
            document.clamp(Position { line: 9, column: 9 }),
            Position { line: 0, column: 4 }
        );
    }
}
