use super::Text;
use crate::enums::{
    ReservedWordEnum, ReservedWordGroup, SymbolEnum, SymbolGroupEnum,
};

impl Text {
    pub(crate) fn starts_with_int(&mut self) -> bool {
        self.next_char()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_letter(&mut self) -> bool {
        self.next_char()
            .map(|c| c.is_alphabetic() || c == '_')
            .unwrap_or_else(|_| self.next_char().map(|c| c == '_').unwrap_or(false))
    }

    pub(crate) fn starts_with_operator(&mut self) -> bool {
        self.next_char()
            .map(|c| {
                let s = c.to_string();
                SymbolGroupEnum::OpComparison.is_prefix(&s)
                    || SymbolGroupEnum::OpArith.is_prefix(&s)
                    || SymbolGroupEnum::OpAssignment.is_prefix(&s)
                    || SymbolGroupEnum::OpBool.is_prefix(&s)
            })
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_bracket_open(&mut self) -> bool {
        self.next_char()
            .map(|c| SymbolEnum::BracketNormOpen.is_prefix(&c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_bracket_closed(&mut self) -> bool {
        self.next_char()
            .map(|c| SymbolEnum::BracketNormClose.is_prefix(&c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_comma(&mut self) -> bool {
        self.next_char()
            .map(|c| SymbolGroupEnum::Commas.is_prefix(&c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_variable(&mut self) -> bool {
        let pos = self.position;
        let res = if !self.starts_with_letter() {
            false
        } else {
            let word = self.peek_word();
            !ReservedWordEnum::is_word(&word)
                && (!self._is_valid_position(self.position.x, self.position.y)
                    || !self.starts_with_bracket_open())
        };
        self.position = pos;
        res
    }

    pub(crate) fn starts_with_data_type(&mut self) -> bool {
        let pos = self.position;
        let res = if !self.starts_with_letter() {
            false
        } else {
            let word = self.peek_word();
            if let Ok(rw) = word.parse::<ReservedWordEnum>() {
                ReservedWordGroup::is_data_type(rw)
            } else {
                false
            }
        };
        self.position = pos;
        res
    }
}
