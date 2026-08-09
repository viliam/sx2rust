use crate::enums::SxErrorType;
use crate::parsers::SxParser;
use crate::position::Position;
use crate::sx_error::SxError;
use crate::text::Text;
use crate::words::word::Word;

pub struct WordParser;

impl WordParser {
    fn _next_char_position(&self, text: &mut Text) {
        text.next_char_position();
    }
}

impl SxParser<Word> for WordParser {
    fn read(&self, text: &mut Text) -> Result<Word, SxError> {
        self._next_char_position(text);
        let position = text.position;
        let word_content = text.peek_word();

        if word_content.is_empty() {
            return Err(SxError::create(
                SxErrorType::ExpectedToken,
                text.position,
                text.line(),
            ));
        }

        let end_x = position.x + word_content.len();
        text.position = Position::new(end_x, position.y);
        Ok(Word::new(position, word_content))
    }
}

pub struct WordExpressionParser;

impl SxParser<Word> for WordExpressionParser {
    fn read(&self, text: &mut Text) -> Result<Word, SxError> {
        text.next_char_position();
        let position = text.position;
        let word_content = text.peek_word();

        if word_content.is_empty() {
            return Err(SxError::create(
                SxErrorType::ExpectedToken,
                text.position,
                text.line(),
            ));
        }

        let end_x = position.x + word_content.len();
        text.position = Position::new(end_x, position.y);
        Ok(Word::new(position, word_content))
    }
}

