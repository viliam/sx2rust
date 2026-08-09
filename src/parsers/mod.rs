use crate::sx_error::SxError;
use crate::enums::{SxErrorType, SymbolGroupEnum};
use crate::position::Position;
use crate::text::Text;
use crate::words::expresions::{
    BracketExpression, Expression, ExpressionABC, Integer, Variable,
};
use crate::words::symbol::{Bracket, Operator};
use crate::words::word::Word;

pub trait SxParser<T> {
    fn read(&self, text: &mut Text) -> Result<T, SxError>;
}

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

pub struct BracketParser;

impl SxParser<Bracket> for BracketParser {
    fn read(&self, text: &mut Text) -> Result<Bracket, SxError> {
        let poz = text.next_char_position().ok_or_else(|| {
            SxError::create_no_msg(SxErrorType::EndOfFile, text.position)
        })?;

        let s = text.take_end_of_line()?;
        let mut symbols: Vec<_> = SymbolGroupEnum::Bracket.members().into_iter().collect();
        symbols.sort_by_key(|s| std::cmp::Reverse(s.as_str().len()));

        for sym in symbols {
            if s.starts_with(sym.as_str()) {
                text.position = poz.add_x(sym.as_str().len());
                return Ok(Bracket::new(poz, sym));
            }
        }

        Err(SxError::create(
            SxErrorType::ExpectedBracket,
            text.position,
            text.line(),
        ))
    }
}

pub struct OperatorExpressionParser;

impl SxParser<Operator> for OperatorExpressionParser {
    fn read(&self, text: &mut Text) -> Result<Operator, SxError> {
        let poz = match text.next_char_position() {
            Some(position) => position,
            None => {
                return Err(SxError::create_no_msg(
                    SxErrorType::EndOfFile,
                    text.position,
                ));
            }
        };

        let s = text.take_end_of_line()?;
        let mut symbols: Vec<_> = SymbolGroupEnum::OpExp.members().into_iter().collect();
        symbols.sort_by_key(|s| std::cmp::Reverse(s.as_str().len()));

        for sym in symbols {
            if s.starts_with(sym.as_str()) {
                text.position = poz.add_x(sym.as_str().len());
                return Ok(Operator::new(poz, sym));
            }
        }

        Err(SxError::create(
            SxErrorType::ExpectedOperator,
            text.position,
            text.line(),
        ))
    }
}

pub struct IntegerParser;

impl SxParser<Integer> for IntegerParser {
    fn read(&self, text: &mut Text) -> Result<Integer, SxError> {
        let int_word = WordExpressionParser.read(text)?;
        let a_int = int_word.content.clone();
        if !a_int.chars().all(|c| c.is_ascii_digit()) {
            return Err(SxError::create(
                SxErrorType::ExpectedInt,
                text.position,
                text.line(),
            ));
        }
        let val = a_int.parse::<i32>().map_err(|_| {
            SxError::create(SxErrorType::ExpectedInt, text.position, text.line())
        })?;
        Ok(Integer::new(val, int_word))
    }
}

pub struct VariableParser;

impl SxParser<Variable> for VariableParser {
    fn read(&self, text: &mut Text) -> Result<Variable, SxError> {
        let word = WordExpressionParser.read(text)?;
        Ok(Variable::new(word))
    }
}

pub struct SimpleExpressionParser;

impl SxParser<Box<dyn ExpressionABC>> for SimpleExpressionParser {
    fn read(&self, text: &mut Text) -> Result<Box<dyn ExpressionABC>, SxError> {
        if text.starts_with_int() {
            return Ok(Box::new(IntegerParser.read(text)?));
        }
        if text.starts_with_variable() {
            return Ok(Box::new(VariableParser.read(text)?));
        }
        Err(SxError::create(
            SxErrorType::UnexpectedPrefix,
            text.position,
            text.line(),
        ))
    }
}

pub struct ExpressionParser;

impl SxParser<Box<dyn ExpressionABC>> for ExpressionParser {
    fn read(&self, text: &mut Text) -> Result<Box<dyn ExpressionABC>, SxError> {
        let expr = if text.starts_with_bracket_open() {
            BracketExpressionParser.read(text)?
        } else {
            SimpleExpressionParser.read(text)?
        };

        if text.is_at_eof() {
            return Ok(expr);
        }

        if text.starts_with_operator() {
            let op = OperatorExpressionParser.read(text)?;
            let right = ExpressionParser.read(text)?;
            return Ok(Box::new(Expression::new(expr, op, right)));
        }

        if !text.is_at_eof()
            && !text.starts_with_operator()
            && !text.starts_with_comma()
            && !text.starts_with_bracket_closed()
        {
            return Err(SxError::create(
                SxErrorType::UncorrectedEndOfExpression,
                text.position,
                text.line(),
            ));
        }

        Ok(expr)
    }
}

pub struct BracketExpressionParser;

impl SxParser<Box<dyn ExpressionABC>> for BracketExpressionParser {
    fn read(&self, text: &mut Text) -> Result<Box<dyn ExpressionABC>, SxError> {
        let z1 = BracketParser.read(text)?;
        let ex = ExpressionParser.read(text)?;
        let z2 = BracketParser.read(text)?;

        Ok(Box::new(BracketExpression::new(z1, ex, z2)))
    }
}
