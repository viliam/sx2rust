use crate::sx_error::SxError;
use crate::enums::ExpType;
use crate::position::Position;
use crate::words::symbol::{Bracket, Operator};
use crate::words::word::{Word, WordABC};

pub trait ExpressionABC: WordABC {
    fn exp_type(&self) -> Result<ExpType, SxError>;
}

#[derive(Debug)]
pub struct WordExpression {
    pub word: Word,
}

impl WordExpression {
    pub fn new(word: Word) -> Self {
        Self { word }
    }
}

impl WordABC for WordExpression {
    fn content(&self) -> String {
        self.word.content.clone()
    }
    fn position(&self) -> Position {
        self.word.position
    }
}

#[derive(Debug)]
pub struct Variable {
    pub word_exp: WordExpression,
}

impl Variable {
    pub fn new(word: Word) -> Self {
        Self {
            word_exp: WordExpression::new(word),
        }
    }
}

impl WordABC for Variable {
    fn content(&self) -> String {
        self.word_exp.content()
    }
    fn position(&self) -> Position {
        self.word_exp.position()
    }
}

impl ExpressionABC for Variable {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        Ok(ExpType::Unknown)
    }
}

#[derive(Debug)]
pub struct DataType {
    pub word_exp: WordExpression,
}

impl DataType {
    pub fn new(word: Word) -> Self {
        Self {
            word_exp: WordExpression::new(word),
        }
    }
}

impl WordABC for DataType {
    fn content(&self) -> String {
        self.word_exp.content()
    }
    fn position(&self) -> Position {
        self.word_exp.position()
    }
}

impl ExpressionABC for DataType {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        Ok(ExpType::Void)
    }
}

#[derive(Debug)]
pub struct Integer {
    pub word_exp: WordExpression,
    pub value: i32,
}

impl Integer {
    pub fn new(value: i32, word: Word) -> Self {
        Self {
            word_exp: WordExpression::new(word),
            value,
        }
    }
}

impl WordABC for Integer {
    fn content(&self) -> String {
        self.word_exp.content()
    }
    fn position(&self) -> Position {
        self.word_exp.position()
    }
}

impl ExpressionABC for Integer {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        Ok(ExpType::Int)
    }
}

#[derive(Debug)]
pub struct Expression {
    pub v1: Box<dyn ExpressionABC>,
    pub op: Operator,
    pub v2: Box<dyn ExpressionABC>,
}

impl Expression {
    pub fn new(v1: Box<dyn ExpressionABC>, op: Operator, v2: Box<dyn ExpressionABC>) -> Self {
        Self { v1, op, v2 }
    }
}

impl WordABC for Expression {
    fn content(&self) -> String {
        format!("{}{}{}", self.v1.content(), self.op.content(), self.v2.content())
    }
    fn position(&self) -> Position {
        self.v1.position()
    }
}

impl ExpressionABC for Expression {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        let t1 = self.v1.exp_type()?;
        let t2 = self.v2.exp_type()?;
        let top = self.op.exp_type();

        let mut types = vec![t1, t2, top];
        types.sort_by_key(|t| t.priority());
        Ok(*types.last().unwrap())
    }
}

#[derive(Debug)]
pub struct BracketExpression {
    pub z1: Bracket,
    pub expression: Box<dyn ExpressionABC>,
    pub z2: Bracket,
}

impl BracketExpression {
    pub fn new(z1: Bracket, expression: Box<dyn ExpressionABC>, z2: Bracket) -> Self {
        Self { z1, expression, z2 }
    }
}

impl WordABC for BracketExpression {
    fn content(&self) -> String {
        format!(
            "{}{}{}",
            self.z1.content(),
            self.expression.content(),
            self.z2.content()
        )
    }
    fn position(&self) -> Position {
        self.z1.position()
    }
}

impl ExpressionABC for BracketExpression {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        self.expression.exp_type()
    }
}
