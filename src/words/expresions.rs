use crate::sx_error::SxError;
use crate::enums::ExpType;
use crate::position::Position;
use crate::words::symbol::{Bracket, Dot, Operator};
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

#[derive(Debug)]
pub struct Parameters {
    pub position: Position,
    pub parameters: Vec<Box<dyn ExpressionABC>>
}

impl Parameters {
    pub fn new(position: Position, parameters: Vec<Box<dyn ExpressionABC>>) -> Self {
        Self { position, parameters }
    }
}

impl WordABC for Parameters {
    fn content(&self) -> String {
        self.parameters.iter().map(|p| p.content()).collect::<Vec<_>>().join(",")
    }

    fn position(&self) -> Position {
        self.position
    }
}

#[derive(Debug)]
pub struct CommandPostfix {
    pub bracket_open: Bracket,
    pub parameters: Parameters,
    pub bracket_close: Bracket,
}

impl CommandPostfix {
    pub fn new(bracket_open: Bracket, parameters: Parameters, bracket_close: Bracket) -> Self {
        Self { bracket_open, parameters, bracket_close }
    }
}

impl WordABC for CommandPostfix {
    fn content(&self) -> String {
        format!("{}{}{}", self.bracket_open.content(), self.parameters.content(), self.bracket_close.content())
    }

    fn position(&self) -> Position {
        self.bracket_open.position()
    }
}

#[derive(Debug)]
pub struct Command {
    pub expression: Box<dyn ExpressionABC>,
    pub command_postfix: CommandPostfix,
}

impl Command {
    pub fn new(expression: Box<dyn ExpressionABC>, command_postfix: CommandPostfix) -> Self {
        Self { expression, command_postfix }
    }
}

#[derive(Debug)]
pub struct ArrayPostfix {
    pub bracket_open: Bracket,
    pub elements: Vec<Box<dyn ExpressionABC>>,
    pub bracket_close: Bracket,
}

impl ArrayPostfix {
    pub fn new(bracket_open: Bracket, elements: Vec<Box<dyn ExpressionABC>>, bracket_close: Bracket) -> Self {
        Self { bracket_open, elements, bracket_close }
    }
}

impl WordABC for ArrayPostfix {

    fn content(&self) -> String {
        format!("{}{}{}", self.bracket_open.content(),
                self.elements.iter().map(|e| e.content()).collect::<Vec<_>>().join(","),
                self.bracket_close.content())
    }

    fn position(&self) -> Position {
        self.bracket_open.position()
    }
}

impl ExpressionABC for ArrayPostfix {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        Ok(ExpType::Unknown)
    }
}

#[derive(Debug)]
pub struct Array {
    pub expression: Box<dyn ExpressionABC>,
    pub array_postfix: ArrayPostfix,
}

impl Array {
    pub fn new(expression: Box<dyn ExpressionABC>, array_postfix: ArrayPostfix) -> Self {
        Self { expression, array_postfix }
    }
}

impl WordABC for Array {
    fn content(&self) -> String {
        format!("{}{}", self.expression.content(), self.array_postfix.content())

    }

    fn position(&self) -> Position {
        self.expression.position()
    }
}

impl ExpressionABC for Array {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        Ok(ExpType::Unknown)
    }
}

#[derive(Debug)]
pub struct MemberAccessPostfix {
    pub dot: Dot,
    pub name: Word,
}

impl MemberAccessPostfix {
    pub fn new(dot: Dot, name: Word) -> Self {
        Self { dot, name }
    }
}

impl WordABC for MemberAccessPostfix {
    fn content(&self) -> String {
        format!("{}{}", self.dot.content(),
                self.name.content())
    }

    fn position(&self) -> Position { self.dot.position()}
}

#[derive(Debug)]
pub struct MemberAccess {
    pub expression: Box<dyn ExpressionABC>,
    pub member_access_postfix: MemberAccessPostfix,
}

impl MemberAccess {
    pub fn new(expression: Box<dyn ExpressionABC>, member_access_postfix: MemberAccessPostfix) -> Self {
        Self { expression, member_access_postfix }
    }
}

impl ExpressionABC for MemberAccess {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        Ok(ExpType::Unknown)
    }
}

impl WordABC for MemberAccess {
    fn content(&self) -> String {
        format!("{}{}", self.expression.content(),
                self.member_access_postfix.content())
    }

    fn position(&self) -> Position { self.expression.position() }
}

impl WordABC for Command {
    fn content(&self) -> String {
        format!("{}{}", self.expression.content(), self.command_postfix.content())
    }

    fn position(&self) -> Position {
        self.expression.position()
    }
}

impl ExpressionABC for Command {
    fn exp_type(&self) -> Result<ExpType, SxError> {
        Ok(ExpType::Unknown)
    }
}