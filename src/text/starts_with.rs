use super::Text;
use crate::enums::ReservedWordEnum;
use crate::words::symbol::{BracketKind, OperatorGroup, SeparatorKind, SymbolKind};

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
                OperatorGroup::Comparison.is_prefix(&s)
                    || OperatorGroup::Arithmetic.is_prefix(&s)
                    || OperatorGroup::Assignment.is_prefix(&s)
                    || OperatorGroup::Boolean.is_prefix(&s)
            })
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_bracket_open(&mut self) -> bool {
        self.next_char()
            .map(|c| BracketKind::RoundOpen.is_prefix(&*c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_bracket_closed(&mut self) -> bool {
        self.next_char()
            .map(|c| BracketKind::RoundClose.is_prefix(&c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_square_bracket_open(&mut self) -> bool {
        self.next_char()
            .map(|c| BracketKind::SquareOpen.is_prefix(&*c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_square_bracket_closed(&mut self) -> bool {
        self.next_char()
            .map(|c| BracketKind::SquareClose.is_prefix(&c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_comma(&mut self) -> bool {
        self.next_char()
            .map(|c| SeparatorKind::Comma.is_prefix(&c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_dot(&mut self) -> bool {
        self.next_char()
            .map(|c| SeparatorKind::Dot.is_prefix(&c.to_string()))
            .unwrap_or(false)
    }

    pub(crate) fn starts_with_command_postfix(&mut self) -> bool {
        self.starts_with_bracket_open()
    }

    pub(crate) fn starts_with_array_postfix(&mut self) -> bool {
        self.starts_with_square_bracket_open()
    }

    pub(crate) fn starts_with_member_access_postfix(&mut self) -> bool {
        self.starts_with_dot()
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


}
