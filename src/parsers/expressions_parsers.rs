use crate::enums::SxErrorType;
use crate::parsers::symbols_parsers::{BracketParser, CommaParser, OperatorExpressionParser};
use crate::parsers::word_parsers::WordExpressionParser;
use crate::parsers::SxParser;
use crate::sx_error::SxError;
use crate::text::Text;
use crate::words::expresions::{BracketExpression, Command, CommandPostfix, Array, ArrayPostfix, Expression, ExpressionABC, Integer, Parameters, Variable};

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
        let mut expr = if text.starts_with_bracket_open() {
            BracketExpressionParser.read(text)?
        } else if text.starts_with_square_bracket_open() {
            Box::new(ArrayPostfixParser.read(text)?)
        } else {
            SimpleExpressionParser.read(text)?
        };

        expr = ExpressionParser::read_postfixes(text, expr)?;

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
            && !text.starts_with_square_bracket_closed()
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

impl ExpressionParser {
    fn read_postfixes(text: &mut Text, baseExpression: Box<dyn ExpressionABC>) -> Result<Box<dyn ExpressionABC>, SxError> {
        let mut expr = baseExpression;
        while !text.is_at_eof()
            && (text.starts_with_command_postfix()
            // || text.is_prefix_member_access_postfix()
            || text.starts_with_array_postfix())
        {
            expr = if text.starts_with_command_postfix() {
                Box::new(Command::new(expr, CommandPostfixParser.read(text)?))
            // }
            // } else if text.is_prefix_member_access_postfix() {
            //     Expression::MemberAccess(Box::new(expr), MemberAccessPostfixParser::instance().read(text)?)
            } else {
                Box::new( Array::new(expr, ArrayPostfixParser.read(text)?))
            }
        }
        return Ok(expr);
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

pub struct ParameterParser;

impl SxParser<Parameters> for ParameterParser {
    fn read(&self, text: &mut Text) -> Result<Parameters, SxError> {
        let mut parameters :Vec<Box<dyn ExpressionABC>> = Vec::new();
        let pos = text.position;

        loop {
            parameters.push(ExpressionParser.read(text)?);
            if text.starts_with_bracket_closed() {
                return Ok(Parameters::new(pos, parameters))
            }
            if !text.starts_with_comma() {
                return Err(SxError::create(
                    SxErrorType::ExpectedParameter,
                    text.position,
                    text.line(),
                ));
            }

            CommaParser.read(text)?;
        }
    }
}

pub struct CommandPostfixParser;

impl SxParser<CommandPostfix> for CommandPostfixParser {
    fn read(&self, text: &mut Text) -> Result<CommandPostfix, SxError> {
        let bracket_open = BracketParser.read(text)?;
        let parameters = ParameterParser.read(text)?;
        let bracket_close = BracketParser.read(text)?;
        Ok(CommandPostfix::new(bracket_open, parameters, bracket_close))
    }
}

pub struct ArrayPostfixParser;

impl SxParser<ArrayPostfix> for ArrayPostfixParser {
    fn read(&self, text: &mut Text) -> Result<ArrayPostfix, SxError> {
        let bracket_open = BracketParser.read(text)?;
        let mut elements :Vec<Box<dyn ExpressionABC>> = Vec::new();

        while !text.starts_with_square_bracket_closed() {
            elements.push( ExpressionParser.read(text)?);
            if text.starts_with_comma() {
                CommaParser.read(text)?;
            }
        }
        let bracket_close = BracketParser.read(text)?;
        Ok(ArrayPostfix::new(bracket_open, elements, bracket_close))
    }
}

// class ArrayPostfixParser : SxParser<ArrayPostfix> {
//
//     val bracketOpen = BracketParser.read(text)
//
//     val elements = mutableListOf<ExpressionAbstract>()
//     while  (!text.startsWithSquareBracketClose()) {
//         elements.add(ExpressionParser.i().read(text))
//         if (text.startsWithComma() )
//             CommaParser.read(text)
//     }
//
//     val bracketClose = BracketParser.read(text)
//     return ArrayPostfix(bracketOpen, elements, bracketClose)
//
// }