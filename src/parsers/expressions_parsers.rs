use crate::enums::SxErrorType;
use crate::parsers::symbols_parsers::{BracketParser, OperatorExpressionParser};
use crate::parsers::word_parsers::WordExpressionParser;
use crate::parsers::SxParser;
use crate::sx_error::SxError;
use crate::text::Text;
use crate::words::expresions::{BracketExpression, Command, Expression, ExpressionABC, Integer, Variable};

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
        } else {
            SimpleExpressionParser.read(text)?
        };

        // expr = ExpressionParser::read_postfixes(text, expr)?;

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

// impl ExpressionParser {
//     fn read_postfixes(text: &mut Text, baseExpression: Box<dyn ExpressionABC>) -> Result<Box<dyn ExpressionABC>, SxError> {
//         let mut expr = baseExpression;
//         while !text.is_at_eof()
//             && (text.starts_with_command_postfix())
//             // || text.is_prefix_member_access_postfix()
//             // || text.is_prefix_array_postfix())
//         {
//             expr = if text.starts_with_command_postfix() {
//                 Command::new(expr, CommandPostfixParser::instance().read(text)?)
//             }
//             // } else if text.is_prefix_member_access_postfix() {
//             //     Expression::MemberAccess(Box::new(expr), MemberAccessPostfixParser::instance().read(text)?)
//             // } else {
//             //     Expression::Array(Box::new(expr), ArrayPostfixParser::instance().read(text)?)
//             // };
//         }
//     }
// }

pub struct BracketExpressionParser;

impl SxParser<Box<dyn ExpressionABC>> for BracketExpressionParser {
    fn read(&self, text: &mut Text) -> Result<Box<dyn ExpressionABC>, SxError> {
        let z1 = BracketParser.read(text)?;
        let ex = ExpressionParser.read(text)?;
        let z2 = BracketParser.read(text)?;

        Ok(Box::new(BracketExpression::new(z1, ex, z2)))
    }
}





// pub struct OperatorExpressionParser;
//
// impl SxParser<Operator> for OperatorExpressionParser {
//     fn read(&self, text: &mut Text) -> Result<Operator, SxError> {
//         let poz = match text.next_char_position() {
//             Some(position) => position,
//             None => {
//                 return Err(SxError::create_no_msg(
//                     SxErrorType::EndOfFile,
//                     text.position,
//                 ));
//             }
//         };
//
//         let s = text.take_end_of_line()?;
//         let mut symbols: Vec<_> = SymbolGroupEnum::OpExp.members().into_iter().collect();
//         symbols.sort_by_key(|s| std::cmp::Reverse(s.as_str().len()));
//
//         for sym in symbols {
//             if s.starts_with(sym.as_str()) {
//                 text.position = poz.add_x(sym.as_str().len());
//                 return Ok(Operator::new(poz, sym));
//             }
//         }
//
//         Err(SxError::create(
//             SxErrorType::ExpectedOperator,
//             text.position,
//             text.line(),
//         ))
//     }
// }