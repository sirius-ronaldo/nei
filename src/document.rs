use std::io;
use std::path::Path;

/// Posição lógica no documento. A coluna é contada em caracteres Unicode.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Document {
    lines: Vec<String>,
    pub modified: bool,
}

impl Document {
    pub fn empty() -> Self {
        Self {
            lines: vec![String::new()],
            modified: false,
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
        Self {
            lines,
            modified: false,
        }
    }

    pub fn from_path(path: &Path) -> io::Result<Self> {
        Ok(Self::from_text(&std::fs::read_to_string(path)?))
    }

    pub fn save_to_path(&mut self, path: &Path) -> io::Result<()> {
        if self.modified && path.is_file() {
            let backup = path.with_file_name(format!(
                "{}.nbk",
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("file")
            ));
            std::fs::copy(path, backup)?;
        }
        std::fs::write(path, self.as_text())?;
        self.modified = false;
        Ok(())
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

    pub fn insert_text(&mut self, position: Position, text: &str) -> Position {
        if text.is_empty() {
            return self.clamp(position);
        }
        let mut content: Vec<char> = self.as_text().chars().collect();
        let offset = self.position_to_offset(position);
        let insertion: Vec<char> = text.chars().collect();
        content.splice(offset..offset, insertion);
        *self = Self::from_text(&content.iter().collect::<String>());
        self.modified = true;
        self.offset_to_position(offset + text.chars().count())
    }

    pub fn replace_char(&mut self, position: Position, character: char) -> Position {
        let position = self.clamp(position);
        let mut content: Vec<char> = self.as_text().chars().collect();
        let offset = self.position_to_offset(position);
        if offset < content.len() && content[offset] != '\n' {
            content[offset] = character;
        } else {
            content.insert(offset, character);
        }
        *self = Self::from_text(&content.iter().collect::<String>());
        self.modified = true;
        Position {
            line: position.line,
            column: position.column + 1,
        }
    }

    pub fn delete_range(&mut self, start: Position, end: Position) -> Option<String> {
        let start = self.clamp(start);
        let end = self.clamp(end);
        let start_offset = self.position_to_offset(start);
        let end_offset = self.position_to_offset(end);
        if start_offset >= end_offset {
            return None;
        }
        let mut content: Vec<char> = self.as_text().chars().collect();
        let deleted: String = content[start_offset..end_offset].iter().collect();
        content.drain(start_offset..end_offset);
        *self = Self::from_text(&content.iter().collect::<String>());
        self.modified = true;
        Some(deleted)
    }

    pub fn text_range(&self, start: Position, end: Position) -> Option<String> {
        let start_offset = self.position_to_offset(start);
        let end_offset = self.position_to_offset(end);
        if start_offset >= end_offset {
            return None;
        }
        Some(
            self.as_text()
                .chars()
                .skip(start_offset)
                .take(end_offset - start_offset)
                .collect(),
        )
    }

    pub fn offset_of(&self, position: Position) -> usize {
        self.position_to_offset(position)
    }

    pub fn position_at_offset(&self, offset: usize) -> Position {
        self.offset_to_position(offset)
    }

    pub fn as_text(&self) -> String {
        self.lines.join("\n")
    }

    fn position_to_offset(&self, position: Position) -> usize {
        let position = self.clamp(position);
        self.lines
            .iter()
            .take(position.line)
            .map(|line| line.chars().count() + 1)
            .sum::<usize>()
            + position.column
    }

    fn offset_to_position(&self, offset: usize) -> Position {
        let mut remaining = offset;
        for (line, text) in self.lines.iter().enumerate() {
            let length = text.chars().count();
            if remaining <= length {
                return Position {
                    line,
                    column: remaining,
                };
            }
            remaining -= length + 1;
        }
        self.clamp(Position {
            line: self.line_count().saturating_sub(1),
            column: usize::MAX,
        })
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

    #[test]
    fn edits_preserve_lines_and_mark_document_modified() {
        let mut document = Document::from_text("ab\ncd");
        document.insert_text(Position { line: 0, column: 1 }, "X\nY");
        assert_eq!(document.line(0), "aX");
        assert_eq!(document.line(1), "Yb");
        assert!(document.modified);
        assert_eq!(
            document.delete_range(
                Position { line: 0, column: 1 },
                Position { line: 1, column: 1 }
            ),
            Some("X\nY".to_owned())
        );
        assert_eq!(document.line(0), "ab");
    }

    #[test]
    fn save_writes_text_and_clears_modified_flag() {
        let path =
            std::env::temp_dir().join(format!("nei-sprint04-save-{}.txt", std::process::id()));
        let mut document = Document::from_text("conteúdo\nfinal");
        document.modified = true;
        document.save_to_path(&path).expect("file should be saved");
        assert_eq!(
            std::fs::read_to_string(&path).expect("file should be readable"),
            "conteúdo\nfinal"
        );
        assert!(!document.modified);
        std::fs::remove_file(path).expect("temporary file should be removed");
    }

    #[test]
    fn saving_modified_existing_file_creates_nbk_backup() {
        let path =
            std::env::temp_dir().join(format!("nei-sprint04-backup-{}.txt", std::process::id()));
        let backup = path.with_file_name(format!(
            "{}.nbk",
            path.file_name().and_then(|name| name.to_str()).unwrap()
        ));
        std::fs::write(&path, "versão antiga").expect("original should be created");
        let mut document = Document::from_text("versão nova");
        document.modified = true;

        document.save_to_path(&path).expect("file should be saved");

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "versão nova");
        assert_eq!(std::fs::read_to_string(&backup).unwrap(), "versão antiga");
        std::fs::remove_file(path).expect("original should be removed");
        std::fs::remove_file(backup).expect("backup should be removed");
    }
}
