use crate::enums::SxErrorType;
use crate::position::Position;
use std::fmt;

#[derive(Debug)]
pub struct SxError {
    pub typ: SxErrorType,
    pub message: String,
    pub position: Position,
}

impl SxError {
    pub fn new(typ: SxErrorType, message: String, position: Position) -> Self {
        Self {
            typ,
            message,
            position,
        }
    }

    pub fn create_no_msg(typ: SxErrorType, position: Position) -> Self {
        Self::new(typ, typ.to_string(), position)
    }

    pub fn create(typ: SxErrorType, position: Position, line: &str) -> Self {
        Self::create_with_message(typ, None, position, line)
    }

    pub fn create_with_message(
        typ: SxErrorType,
        message: Option<String>,
        position: Position,
        line: &str,
    ) -> Self {
        let final_message = message.unwrap_or_else(|| Self::_make_message(typ, position, line));
        Self::new(typ, final_message, position)
    }

    pub fn create_with_position(typ: SxErrorType, position: Position) -> Self {
        Self::new(typ, typ.to_string(), position)
    }

    fn _make_message(typ: SxErrorType, position: Position, line: &str) -> String {
        let col = position.x;
        let a_char = line.chars().nth(col).unwrap_or(' ');
        format!("{:?}  : {}    \n char = {}", typ, line, a_char)
    }

    #[cfg(test)]
    pub fn test_make_message() {
        let position = Position::new(4, 0);
        let line = "Example line";
        let result = Self::_make_message(SxErrorType::UnexpectedPrefix, position, line);
        // In Rust, {:?} for UnexpectedPrefix will likely be "UnexpectedPrefix"
        let expected = "UnexpectedPrefix  : Example line    \n char = p";
        assert_eq!(expected, result);
    }
}

impl fmt::Display for SxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}  \n row = {} ,   column = {}",
            self.message, self.position.y, self.position.x
        )
    }
}

impl std::error::Error for SxError {}
