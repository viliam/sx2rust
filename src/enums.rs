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
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
// pub enum SymbolEnum {
//     Plus,
//     Minus,
//     Times,
//     Divide,
//     Modulo,
//     FloorDivision,
//     Exponent,
//     And,
//     Or,
//     AndStrong,
//     OrStrong,
//     Smaller,
//     Greater,
//     SmallerEqual,
//     GreaterEqual,
//     Equal,
//     Unequal,
//     Assign,
//     BracketNormOpen,
//     BracketNormClose,
//     ParenthesisBlockOpen,
//     ParenthesisBlockClose,
//     Comma,
//     Semicolon,
//     Dot,
// }
//
// impl SymbolEnum {
//     pub fn as_str(&self) -> &'static str {
//         match self {
//             SymbolEnum::Plus => "+",
//             SymbolEnum::Minus => "-",
//             SymbolEnum::Times => "*",
//             SymbolEnum::Divide => "/",
//             SymbolEnum::Modulo => "%",
//             SymbolEnum::FloorDivision => "//",
//             SymbolEnum::Exponent => "**",
//             SymbolEnum::And => "and",
//             SymbolEnum::Or => "or",
//             SymbolEnum::AndStrong => "&&",
//             SymbolEnum::OrStrong => "||",
//             SymbolEnum::Smaller => "<",
//             SymbolEnum::Greater => ">",
//             SymbolEnum::SmallerEqual => "<=",
//             SymbolEnum::GreaterEqual => ">=",
//             SymbolEnum::Equal => "==",
//             SymbolEnum::Unequal => "!=",
//             SymbolEnum::Assign => "=",
//             SymbolEnum::BracketNormOpen => "(",
//             SymbolEnum::BracketNormClose => ")",
//             SymbolEnum::ParenthesisBlockOpen => "{",
//             SymbolEnum::ParenthesisBlockClose => "}",
//             SymbolEnum::Comma => ",",
//             SymbolEnum::Semicolon => ";",
//             SymbolEnum::Dot => ".", // Python code has DOT = "," which seems like a typo, but I'll use "."
//         }
//     }
//
//     pub fn symbol(&self) -> &'static str {
//         self.as_str()
//     }
//
//     pub fn make_symbol(s: &str) -> Option<Self> {
//         match s {
//             "+" => Some(SymbolEnum::Plus),
//             "-" => Some(SymbolEnum::Minus),
//             "*" => Some(SymbolEnum::Times),
//             "/" => Some(SymbolEnum::Divide),
//             "%" => Some(SymbolEnum::Modulo),
//             "//" => Some(SymbolEnum::FloorDivision),
//             "**" => Some(SymbolEnum::Exponent),
//             "and" => Some(SymbolEnum::And),
//             "or" => Some(SymbolEnum::Or),
//             "&&" => Some(SymbolEnum::AndStrong),
//             "||" => Some(SymbolEnum::OrStrong),
//             "<" => Some(SymbolEnum::Smaller),
//             ">" => Some(SymbolEnum::Greater),
//             "<=" => Some(SymbolEnum::SmallerEqual),
//             ">=" => Some(SymbolEnum::GreaterEqual),
//             "==" => Some(SymbolEnum::Equal),
//             "!=" => Some(SymbolEnum::Unequal),
//             "=" => Some(SymbolEnum::Assign),
//             "(" => Some(SymbolEnum::BracketNormOpen),
//             ")" => Some(SymbolEnum::BracketNormClose),
//             "{" => Some(SymbolEnum::ParenthesisBlockOpen),
//             "}" => Some(SymbolEnum::ParenthesisBlockClose),
//             "," => Some(SymbolEnum::Comma),
//             ";" => Some(SymbolEnum::Semicolon),
//             "." => Some(SymbolEnum::Dot),
//             _ => None,
//         }
//     }
//
//     pub fn is_prefix(&self, a: &str) -> bool {
//         if a.is_empty() {
//             return false;
//         }
//         self.as_str().starts_with(&a[0..1])
//     }
// }
//
// impl std::fmt::Display for SymbolEnum {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self.as_str())
//     }
// }
//
// pub enum SymbolGroupEnum {
//     OpArith,
//     OpBool,
//     OpComparison,
//     OpExp,
//     OpAssignment,
//     Commas,
//     Bracket,
// }
//
// impl SymbolGroupEnum {
//     pub fn members(&self) -> HashSet<SymbolEnum> {
//         match self {
//             SymbolGroupEnum::OpArith => [
//                 SymbolEnum::Plus,
//                 SymbolEnum::Minus,
//                 SymbolEnum::Times,
//                 SymbolEnum::Modulo,
//                 SymbolEnum::Divide,
//                 SymbolEnum::FloorDivision,
//                 SymbolEnum::Exponent,
//             ]
//             .into_iter()
//             .collect(),
//             SymbolGroupEnum::OpBool => [
//                 SymbolEnum::And,
//                 SymbolEnum::AndStrong,
//                 SymbolEnum::Or,
//                 SymbolEnum::OrStrong,
//             ]
//             .into_iter()
//             .collect(),
//             SymbolGroupEnum::OpComparison => [
//                 SymbolEnum::Smaller,
//                 SymbolEnum::SmallerEqual,
//                 SymbolEnum::Greater,
//                 SymbolEnum::GreaterEqual,
//                 SymbolEnum::Equal,
//                 SymbolEnum::Unequal,
//             ]
//             .into_iter()
//             .collect(),
//             SymbolGroupEnum::OpExp => [
//                 SymbolEnum::Plus,
//                 SymbolEnum::Minus,
//                 SymbolEnum::Times,
//                 SymbolEnum::Modulo,
//                 SymbolEnum::Divide,
//                 SymbolEnum::FloorDivision,
//                 SymbolEnum::Exponent,
//                 SymbolEnum::And,
//                 SymbolEnum::AndStrong,
//                 SymbolEnum::Or,
//                 SymbolEnum::OrStrong,
//                 SymbolEnum::Smaller,
//                 SymbolEnum::SmallerEqual,
//                 SymbolEnum::Greater,
//                 SymbolEnum::GreaterEqual,
//                 SymbolEnum::Equal,
//                 SymbolEnum::Unequal,
//             ]
//             .into_iter()
//             .collect(),
//             SymbolGroupEnum::OpAssignment => [SymbolEnum::Assign].into_iter().collect(),
//             SymbolGroupEnum::Commas => [SymbolEnum::Comma, SymbolEnum::Semicolon, SymbolEnum::Dot]
//                 .into_iter()
//                 .collect(),
//             SymbolGroupEnum::Bracket => [
//                 SymbolEnum::BracketNormOpen,
//                 SymbolEnum::BracketNormClose,
//                 SymbolEnum::ParenthesisBlockOpen,
//                 SymbolEnum::ParenthesisBlockClose,
//             ]
//             .into_iter()
//             .collect(),
//         }
//     }
//
//     pub fn contains(&self, word: SymbolEnum) -> bool {
//         self.members().contains(&word)
//     }
//
//     pub fn is_prefix(&self, a: &str) -> bool {
//         self.members().iter().any(|sy| sy.is_prefix(a))
//     }
// }
