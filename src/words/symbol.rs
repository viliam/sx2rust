use crate::enums::ExpType;
use crate::position::Position;
use crate::words::word::WordABC;

pub trait SymbolKind {
    fn token(&self) -> &'static str;

    fn is_prefix(&self, value: &str) -> bool {
        !value.is_empty() && self.token().starts_with(value)
    }
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub position: Position,
    pub content: String,
}

impl Symbol {
    pub fn new(position: Position, content: String) -> Self {
        Self { position, content }
    }
}

impl WordABC for Symbol {
    fn content(&self) -> String {
        self.content.clone()
    }

    fn position(&self) -> Position {
        self.position
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DotKind {
    Dot,
}

impl SymbolKind for DotKind {
    fn token(&self) -> &'static str {
        match self {
            DotKind::Dot => ".",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Dot {
    pub symbol: Symbol,
    pub kind: DotKind,
}

impl Dot {
    pub fn new(position: Position, kind: DotKind) -> Self {
        Self {
            symbol: Symbol::new(position, kind.token().to_string()),
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SeparatorKind {
    Comma,
    Semicolon,
}

impl SymbolKind for SeparatorKind {
    fn token(&self) -> &'static str {
        match self {
            SeparatorKind::Comma => ",",
            SeparatorKind::Semicolon => ";",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Separator {
    pub symbol: Symbol,
    pub kind: SeparatorKind,
}

impl Separator {
    pub fn new(position: Position, kind: SeparatorKind) -> Self {
        Self {
            symbol: Symbol::new(position, kind.token().to_string()),
            kind,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatorGroup {
    Arithmetic,
    Boolean,
    Comparison,
    Assignment,
}

impl OperatorGroup {
    pub fn exp_type(&self) -> ExpType {
        match self {
            OperatorGroup::Arithmetic => ExpType::Int,
            OperatorGroup::Boolean => ExpType::Bool,
            OperatorGroup::Comparison => ExpType::Comparison,
            OperatorGroup::Assignment => ExpType::Unknown,
        }
    }

    pub fn is_prefix(&self, token: &str) -> bool {
        OperatorKind::all()
            .iter()
            .any(|kind| kind.group() == *self && kind.is_prefix(token))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OperatorKind {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    FloorDivision,
    Exponent,

    And,
    Or,
    AndWeak,
    OrWeak,
    AndStrong,
    OrStrong,

    Equal,
    NotEqual,
    Smaller,
    Greater,
    SmallerEqual,
    GreaterEqual,

    Assign,
}

impl OperatorKind {
    pub fn all() -> &'static [OperatorKind] {
        &[
            OperatorKind::Plus,
            OperatorKind::Minus,
            OperatorKind::Times,
            OperatorKind::Divide,
            OperatorKind::Modulo,
            OperatorKind::FloorDivision,
            OperatorKind::Exponent,
            OperatorKind::And,
            OperatorKind::Or,
            OperatorKind::AndWeak,
            OperatorKind::OrWeak,
            OperatorKind::AndStrong,
            OperatorKind::OrStrong,
            OperatorKind::Equal,
            OperatorKind::NotEqual,
            OperatorKind::Smaller,
            OperatorKind::Greater,
            OperatorKind::SmallerEqual,
            OperatorKind::GreaterEqual,
            OperatorKind::Assign,
        ]
    }

    pub fn group(&self) -> OperatorGroup {
        match self {
            OperatorKind::Plus
            | OperatorKind::Minus
            | OperatorKind::Times
            | OperatorKind::Divide
            | OperatorKind::Modulo
            | OperatorKind::FloorDivision
            | OperatorKind::Exponent => OperatorGroup::Arithmetic,

            OperatorKind::And
            | OperatorKind::Or
            | OperatorKind::AndWeak
            | OperatorKind::OrWeak
            | OperatorKind::AndStrong
            | OperatorKind::OrStrong => OperatorGroup::Boolean,

            OperatorKind::Equal
            | OperatorKind::NotEqual
            | OperatorKind::Smaller
            | OperatorKind::Greater
            | OperatorKind::SmallerEqual
            | OperatorKind::GreaterEqual => OperatorGroup::Comparison,

            OperatorKind::Assign => OperatorGroup::Assignment,
        }
    }
}

impl SymbolKind for OperatorKind {
    fn token(&self) -> &'static str {
        match self {
            OperatorKind::Plus => "+",
            OperatorKind::Minus => "-",
            OperatorKind::Times => "*",
            OperatorKind::Divide => "/",
            OperatorKind::Modulo => "%",
            OperatorKind::FloorDivision => "//",
            OperatorKind::Exponent => "**",

            OperatorKind::And => "and",
            OperatorKind::Or => "or",
            OperatorKind::AndWeak => "&",
            OperatorKind::OrWeak => "|",
            OperatorKind::AndStrong => "&&",
            OperatorKind::OrStrong => "||",

            OperatorKind::Equal => "==",
            OperatorKind::NotEqual => "!=",
            OperatorKind::Smaller => "<",
            OperatorKind::Greater => ">",
            OperatorKind::SmallerEqual => "<=",
            OperatorKind::GreaterEqual => ">=",

            OperatorKind::Assign => "=",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Operator {
    pub symbol: Symbol,
    pub kind: OperatorKind,
}

impl Operator {
    pub fn new(position: Position, kind: OperatorKind) -> Self {
        Self {
            symbol: Symbol::new(position, kind.token().to_string()),
            kind,
        }
    }

    pub fn exp_type(&self) -> ExpType {
        self.kind.group().exp_type()
    }
}

impl WordABC for Operator {
    fn content(&self) -> String {
        self.symbol.content.clone()
    }

    fn position(&self) -> Position {
        self.symbol.position
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BracketKind {
    RoundOpen,
    RoundClose,
    SquareOpen,
    SquareClose,
    CurlyOpen,
    CurlyClose,
}

impl SymbolKind for BracketKind {
    fn token(&self) -> &'static str {
        match self {
            BracketKind::RoundOpen => "(",
            BracketKind::RoundClose => ")",
            BracketKind::SquareOpen => "[",
            BracketKind::SquareClose => "]",
            BracketKind::CurlyOpen => "{",
            BracketKind::CurlyClose => "}",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bracket {
    pub symbol: Symbol,
    pub kind: BracketKind,
}

impl Bracket {
    pub fn new(position: Position, kind: BracketKind) -> Self {
        Self {
            symbol: Symbol::new(position, kind.token().to_string()),
            kind,
        }
    }
}

impl WordABC for Bracket {
    fn content(&self) -> String {
        self.symbol.content.clone()
    }
    fn position(&self) -> Position {
        self.symbol.position()
    }
}

#[derive(Debug, Clone)]
pub struct OpenBracket {
    pub bracket: Bracket,
}

impl OpenBracket {
    pub fn new(position: Position, kind: BracketKind) -> Self {
        Self {
            bracket: Bracket::new(position, kind),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloseBracket {
    pub bracket: Bracket,
}

impl CloseBracket {
    pub fn new(position: Position, kind: BracketKind) -> Self {
        Self {
            bracket: Bracket::new(position, kind),
        }
    }
}