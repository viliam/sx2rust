use crate::sx_error::SxError;
use crate::enums::SxErrorType;
use crate::position::Position;

mod starts_with;

pub struct Text {
    lines: Vec<String>,
    pub position: Position,
}

impl Text {
    pub fn new(lines: Vec<String>) -> Self {
        Self {
            lines,
            position: Position::new(0, 0),
        }
    }

    pub fn line(&self) -> &str {
        &self.lines[self.position.y]
    }

    pub fn next_char_position(&mut self) -> Option<Position> {
        let mut y = self.position.y;
        let mut x = self.position.x;

        while y < self.lines.len() {
            let offset = self.next_char_position_at_line(x, y);

            if let Some(offset) = offset {
                self.position = Position { x: x + offset, y };
                return Some(self.position);
            }

            y += 1;
            x = 0;
        }
        
        None
    }

    fn next_char_position_at_line(&mut self, x: usize, y: usize) -> Option<usize> {
        self.lines[y][x..]
            .char_indices()
            .find(|(_, c)| !c.is_whitespace())
            .map(|(i, _)| i)
    }

    pub fn next_char(&mut self) -> Result<char, SxError> {
        let actual = self.position;
        self.next_char_position();
        self.ensure_not_eof()?;

        let x = self.position.x;
        let y = self.position.y;
        let p = self.lines[y].chars().nth(x).unwrap();
        self.position = actual;
        Ok(p)
    }

    pub fn peek_word(&self) -> String {
        let line = self.line();
        let x = self.position.x;
        let end_x = Self::_find_end_of_word(line, x);
        if x < end_x {
            line[x..end_x].to_string()
        } else {
            String::new()
        }
    }

    fn _find_end_of_word(line: &str, x: usize) -> usize {
        line[x..]
            .char_indices()
            .find(|(_, ch)| !ch.is_alphanumeric())
            .map(|(i, _)| x + i)
            .unwrap_or(line.len())
    }

    pub fn take_end_of_line(&mut self) -> Result<&str, SxError> {
        self.ensure_not_eof()?;
        let x = self.position.x;
        let y = self.position.y;
        Ok(&self.lines[y][x..])
    }

    pub fn is_at_eof(&mut self) -> bool {
        let pos = self.position;
        let res = self.next_char_position().is_none();
        self.position = pos;
        res
    }

    pub fn ensure_not_eof(&mut self) -> Result<(), SxError> {
        if self.is_at_eof() {
            return Err(SxError::create_no_msg(SxErrorType::EndOfFile, self.position));
        }
        Ok(())
    }

    fn _is_valid_position(&self, x: usize, y: usize) -> bool {
        if y >= self.lines.len() {
            return false;
        }
        if x >= self.lines[y].len() {
            return false;
        }
        true
    }

    #[cfg(test)]
    pub fn test_is_valid_position()  {
        let text = Text::new(vec!["la".to_string()]);
        // Rust uses usize, so negative values are not possible to test the same way,
        // but we can test boundaries
        assert!(text._is_valid_position(0, 0));
        assert!(!text._is_valid_position(2, 0));
        assert!(!text._is_valid_position(0, 1));
    }
}
