use crate::enums::SxErrorType;
use crate::parsers::SxParser;
use crate::position::Position;
use crate::sx_error::SxError;
use crate::text::Text;
use crate::words::symbol::{Bracket, BracketKind, Dot, DotKind, Operator, OperatorKind, Separator, SeparatorKind, SymbolKind};

pub type ParseResult<T> = Result<T, SxError>;

struct SymbolParser<T, K, F>
where
    K: SymbolKind + Copy + Eq,
    F: Fn(Position, K) -> T,
{
    symbols: Vec<K>,
    exception_type: SxErrorType,
    create: F,
}

impl<T, K, F> SymbolParser<T, K, F>
where
    K: SymbolKind + Copy + Eq,
    F: Fn(Position, K) -> T,
{
    fn new(symbols: Vec<K>, exception_type: SxErrorType, create: F) -> Self {
        Self {
            symbols,
            exception_type,
            create,
        }
    }

    fn take_symbol(&self, text: &mut Text) -> ParseResult<K> {
        let remaining_text = text.take_end_of_line()?;

        let symbol = self.symbols
            .iter()
            .copied()
            .find(|symbol| remaining_text.starts_with(symbol.token()))
            .ok_or_else(|| {
                SxError::create(
                    self.exception_type,
                    text.position.clone(),
                    text.line(),
                )
            })?;

        text.position = text.position.add_x(symbol.token().len());

        Ok(symbol)
    }
}

impl<T, K, F> SxParser<T> for SymbolParser<T, K, F>
where
    K: SymbolKind + Copy + Eq,
    F: Fn(Position, K) -> T,
{
    fn read(&self, text: &mut Text) -> ParseResult<T> {
        let position = text
            .next_char_position()
            .unwrap_or_else(|| text.position.clone());

        match self.take_symbol(text) {
            Ok(symbol) => Ok((self.create)(position, symbol)),
            Err(err) => Err(SxError::create(
                err.typ,
                text.position.clone(),
                text.line(),
            )),
        }
    }
}

pub struct BracketParser;

impl SxParser<Bracket> for BracketParser {
    fn read(&self, text: &mut Text) -> ParseResult<Bracket> {
        let parser = SymbolParser::new(
            vec![
                BracketKind::RoundOpen,
                BracketKind::RoundClose,
                BracketKind::SquareOpen,
                BracketKind::SquareClose,
                BracketKind::CurlyOpen,
                BracketKind::CurlyClose,
            ],
            SxErrorType::ExpectedBracket,
            |position, kind| match kind {
                BracketKind::RoundOpen
                | BracketKind::SquareOpen
                | BracketKind::CurlyOpen => {
                    Bracket::new(position, kind)
                }

                BracketKind::RoundClose
                | BracketKind::SquareClose
                | BracketKind::CurlyClose => {
                    Bracket::new(position, kind)
                }
            },
        );

        parser.read(text)
    }
}

pub struct CommaParser;

impl SxParser<Separator> for CommaParser {
    fn read(&self, text: &mut Text) -> ParseResult<Separator> {
        let parser = SymbolParser::new(
            vec![SeparatorKind::Comma],
            SxErrorType::ExpectedComma,
            Separator::new,
        );

        parser.read(text)
    }
}

pub struct DotParser;

impl SxParser<Dot> for DotParser {
    fn read(&self, text: &mut Text) -> ParseResult<Dot> {
        let parser = SymbolParser::new(
            vec![DotKind::Dot],
            SxErrorType::ExpectedDot,
            Dot::new,
        );

        parser.read(text)
    }
}

pub struct OperatorExpressionParser;

impl SxParser<Operator> for OperatorExpressionParser {
    fn read(&self, text: &mut Text) -> ParseResult<Operator> {
        let parser = SymbolParser::new(
            OperatorKind::all().to_vec(),
            SxErrorType::ExpectedOperator,
            Operator::new,
        );

        parser.read(text)
    }
}
