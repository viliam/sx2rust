use crate::sx_error::SxError;
use crate::enums::{ExpType, SxErrorType, SymbolEnum};
use crate::position::Position;
use crate::words::word::WordABC;

#[derive(Debug)]
pub struct Symbol {
    pub position: Position,
    pub symbol_enum: SymbolEnum,
}

impl Symbol {
    pub fn new(position: Position, symbol_enum: SymbolEnum) -> Self {
        Self {
            position,
            symbol_enum,
        }
    }
}

impl WordABC for Symbol {
    fn content(&self) -> String {
        self.symbol_enum.symbol().to_string()
    }

    fn position(&self) -> Position {
        self.position
    }
}

#[derive(Debug)]
pub struct Bracket {
    pub symbol: Symbol,
}

impl Bracket {
    pub fn new(position: Position, symbol_enum: SymbolEnum) -> Self {
        Self {
            symbol: Symbol::new(position, symbol_enum),
        }
    }
}

impl WordABC for Bracket {
    fn content(&self) -> String {
        self.symbol.content()
    }

    fn position(&self) -> Position {
        self.symbol.position()
    }
}

#[derive(Debug)]
pub struct Operator {
    pub symbol: Symbol,
}

impl Operator {
    pub fn new(position: Position, symbol_enum: SymbolEnum) -> Self {
        Self {
            symbol: Symbol::new(position, symbol_enum),
        }
    }

    pub fn exp_type(&self) -> Result<ExpType, SxError> {
        match self.symbol.symbol_enum {
            SymbolEnum::Plus
            | SymbolEnum::Minus
            | SymbolEnum::Times
            | SymbolEnum::Modulo
            | SymbolEnum::Divide
            | SymbolEnum::FloorDivision
            | SymbolEnum::Exponent => Ok(ExpType::Int),
            SymbolEnum::And | SymbolEnum::Or | SymbolEnum::AndStrong | SymbolEnum::OrStrong => {
                Ok(ExpType::Bool)
            }
            SymbolEnum::Smaller
            | SymbolEnum::Greater
            | SymbolEnum::SmallerEqual
            | SymbolEnum::GreaterEqual
            | SymbolEnum::Equal
            | SymbolEnum::Unequal => Ok(ExpType::Comparison),
            SymbolEnum::Assign => Ok(ExpType::Unknown),
            _ => Err(SxError::create_with_message(
                SxErrorType::UnknownOperator,
                Some(self.symbol.symbol_enum.to_string()),
                self.symbol.position,
                "",
            )),
        }
    }
}

impl WordABC for Operator {
    fn content(&self) -> String {
        self.symbol.content()
    }

    fn position(&self) -> Position {
        self.symbol.position()
    }
}
