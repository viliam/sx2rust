use crate::position::Position;

use std::fmt::Debug;

pub trait WordABC: Debug {
    fn content(&self) -> String;
    fn position(&self) -> Position;
}

#[derive(Debug)]
pub struct Word {
    pub position: Position,
    pub content: String,
}

impl Word {
    pub fn new(position: Position, content: String) -> Self {
        Self { position, content }
    }

    pub fn from_word(word: &Word) -> Self {
        Self {
            position: word.position,
            content: word.content.clone(),
        }
    }
}

impl WordABC for Word {
    fn content(&self) -> String {
        self.content.clone()
    }

    fn position(&self) -> Position {
        self.position
    }
}

impl std::fmt::Display for Word {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.content)
    }
}
