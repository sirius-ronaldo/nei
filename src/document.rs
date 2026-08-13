use std::io;
use std::path::Path;

/// Posição lógica no documento. A coluna é contada em caracteres Unicode.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TextMatch {
    pub start: usize,
    pub length: usize,
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

    /// Procura um trecho sem ultrapassar o início/fim do documento.
    pub fn find_text(
        &self,
        needle: &str,
        from: usize,
        forward: bool,
        case_sensitive: bool,
    ) -> Option<TextMatch> {
        let needle: Vec<char> = needle.chars().collect();
        if needle.is_empty() {
            return None;
        }
        let haystack: Vec<char> = self.as_text().chars().collect();
        if needle.len() > haystack.len() {
            return None;
        }
        let last = haystack.len() - needle.len();
        let matches = |start: usize| {
            haystack[start..start + needle.len()]
                .iter()
                .zip(&needle)
                .all(|(left, right)| {
                    if case_sensitive {
                        left == right
                    } else {
                        left.to_lowercase().to_string() == right.to_lowercase().to_string()
                    }
                })
        };
        if forward {
            let start = from.min(last);
            (start..=last)
                .find(|&start| matches(start))
                .map(|start| TextMatch {
                    start,
                    length: needle.len(),
                })
        } else {
            let start = from.min(last);
            (0..=start)
                .rev()
                .find(|&start| matches(start))
                .map(|start| TextMatch {
                    start,
                    length: needle.len(),
                })
        }
    }

    pub fn replace_text(&mut self, found: TextMatch, replacement: &str) -> Position {
        let start = self.position_at_offset(found.start);
        let end = self.position_at_offset(found.start + found.length);
        self.delete_range(start, end);
        self.insert_text(start, replacement)
    }

    /// Insere quebras físicas a cada largura informada e devolve o mapa de offsets.
    pub fn wrap_lines(&mut self, width: usize) -> Option<Vec<usize>> {
        if width == 0 {
            return None;
        }
        let original: Vec<char> = self.as_text().chars().collect();
        let mut wrapped = String::new();
        let mut offsets = vec![0; original.len() + 1];
        let mut column = 0;
        for (offset, character) in original.iter().copied().enumerate() {
            offsets[offset] = wrapped.chars().count();
            if character == '\n' {
                wrapped.push(character);
                column = 0;
            } else {
                let starts_word = !character.is_whitespace()
                    && (offset == 0 || original[offset - 1].is_whitespace());
                let word_length = if starts_word {
                    original[offset..]
                        .iter()
                        .take_while(|character| !character.is_whitespace())
                        .count()
                } else {
                    0
                };
                if starts_word && column > 0 && column + word_length > width {
                    wrapped.push('\n');
                    column = 0;
                }
                wrapped.push(character);
                column += 1;
            }
        }
        offsets[original.len()] = wrapped.chars().count();
        if wrapped == self.as_text() {
            return None;
        }
        *self = Self::from_text(&wrapped);
        self.modified = true;
        Some(offsets)
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

    #[test]
    fn searches_utf8_forward_and_backward_without_circularity() {
        let document = Document::from_text("Ação ação fim");
        assert_eq!(
            document.find_text("AÇÃO", 0, true, false),
            Some(TextMatch {
                start: 0,
                length: 4
            })
        );
        assert_eq!(
            document.find_text("ação", 9, false, true),
            Some(TextMatch {
                start: 5,
                length: 4
            })
        );
        assert_eq!(document.find_text("não", 0, true, false), None);
    }

    #[test]
    fn replacement_accepts_empty_text_and_changes_document() {
        let mut document = Document::from_text("um dois dois");
        let found = document.find_text("dois", 0, true, true).unwrap();
        let cursor = document.replace_text(found, "");
        assert_eq!(document.as_text(), "um  dois");
        assert_eq!(cursor, Position { line: 0, column: 3 });
        assert!(document.modified);
    }

    #[test]
    fn physical_wrap_inserts_breaks_without_losing_text() {
        let mut document = Document::from_text("abc defgh\ngh");
        let offsets = document.wrap_lines(3).unwrap();
        assert_eq!(document.as_text(), "abc \ndefgh\ngh");
        assert_eq!(offsets[3], 3);
        assert!(document.modified);
    }

    #[test]
    fn physical_wrap_does_not_split_words() {
        let mut document = Document::from_text("um dois tres");
        document.wrap_lines(6);
        assert_eq!(document.as_text(), "um \ndois \ntres");
    }

    #[test]
    fn long_utf8_lines_keep_character_positions() {
        let text = "áéíóú".repeat(20_000);
        let document = Document::from_text(&text);
        assert_eq!(document.line_count(), 1);
        assert_eq!(document.line_length(0), 100_000);
        assert_eq!(
            document.clamp(Position {
                line: 0,
                column: 100_000
            }),
            Position {
                line: 0,
                column: 100_000
            }
        );
    }

    #[test]
    fn reading_missing_or_invalid_files_returns_an_error() {
        let missing =
            std::env::temp_dir().join(format!("nei-sprint08-missing-{}", std::process::id()));
        assert!(Document::from_path(&missing).is_err());

        let invalid =
            std::env::temp_dir().join(format!("nei-sprint08-invalid-{}", std::process::id()));
        std::fs::write(&invalid, [0xff, 0xfe]).expect("invalid fixture should be created");
        assert!(Document::from_path(&invalid).is_err());
        std::fs::remove_file(invalid).expect("invalid fixture should be removed");
    }

    #[test]
    fn writing_to_a_directory_returns_an_error() {
        let mut document = Document::from_text("conteúdo");
        document.modified = true;
        assert!(document.save_to_path(std::path::Path::new("/tmp")).is_err());
    }
}
