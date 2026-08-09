pub mod symbols_parsers;
pub mod expressions_parsers;
pub mod word_parsers;

use crate::sx_error::SxError;
use crate::text::Text;

pub trait SxParser<T> {
    fn read(&self, text: &mut Text) -> Result<T, SxError>;
}


