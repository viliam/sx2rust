use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SxErrorType {
    UncorrectedEndOfExpression,
    EndOfFile,
    ExpectedInt,
    UnexpectedPrefix,
    ExpectedDataType,
    ExpectedBracket,
    ExpectedOperator,
    ExpectedToken,
    ExpectedComma,
    ExpectedDot,
    ExpectedParameter,
    UnknownOperator,
}

impl std::fmt::Display for SxErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpType {
    Void,
    Int,
    Bool,
    Comparison,
    Unknown,
}

impl ExpType {
    pub fn priority(&self) -> i32 {
        match self {
            ExpType::Void => 0,
            ExpType::Int => 1,
            ExpType::Bool => 3,
            ExpType::Comparison => 2,
            ExpType::Unknown => -1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReservedWordEnum {
    Int,
    Bool,
    Return,
    If,
    Void,
    True,
    False,
    And,
    Or,
}

impl ReservedWordEnum {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReservedWordEnum::Int => "int",
            ReservedWordEnum::Bool => "bool",
            ReservedWordEnum::Return => "return",
            ReservedWordEnum::If => "if",
            ReservedWordEnum::Void => "void",
            ReservedWordEnum::True => "true",
            ReservedWordEnum::False => "false",
            ReservedWordEnum::And => "and",
            ReservedWordEnum::Or => "or",
        }
    }

    pub fn matches(&self, s: &str) -> bool {
        self.as_str() == s
    }

    pub fn symbol(&self) -> &'static str {
        self.as_str()
    }

    pub fn make_symbol(reserved_word: &str) -> Option<Self> {
        Self::from_str(reserved_word).ok()
    }

    pub fn is_word(value: &str) -> bool {
        Self::from_str(value).is_ok()
    }
}

impl FromStr for ReservedWordEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "int" => Ok(ReservedWordEnum::Int),
            "bool" => Ok(ReservedWordEnum::Bool),
            "return" => Ok(ReservedWordEnum::Return),
            "if" => Ok(ReservedWordEnum::If),
            "void" => Ok(ReservedWordEnum::Void),
            "true" => Ok(ReservedWordEnum::True),
            "false" => Ok(ReservedWordEnum::False),
            "and" => Ok(ReservedWordEnum::And),
            "or" => Ok(ReservedWordEnum::Or),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for ReservedWordEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

pub struct ReservedWordGroup;

impl ReservedWordGroup {
    pub fn is_data_type(word: ReservedWordEnum) -> bool {
        matches!(word, ReservedWordEnum::Int | ReservedWordEnum::Bool)
    }

    pub fn is_instruction_word(word: ReservedWordEnum) -> bool {
        matches!(word, ReservedWordEnum::Return | ReservedWordEnum::If)
    }

    pub fn is_data_value(word: ReservedWordEnum) -> bool {
        matches!(
            word,
            ReservedWordEnum::Void | ReservedWordEnum::True | ReservedWordEnum::False
        )
    }
}
