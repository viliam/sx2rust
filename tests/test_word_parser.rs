use sx2rust::enums::SxErrorType;
use sx2rust::parsers::{SxParser, WordParser};
use sx2rust::position::Position;
use sx2rust::text::Text;

#[test]
fn test_read_single_word() {
    let mut text = Text::new(vec!["Hello".to_string()]);
    let word = WordParser.read(&mut text).unwrap();

    assert_eq!(text.position, Position::new(5, 0));
    assert_eq!(word.content, "Hello");
    assert_eq!(word.position, Position::new(0, 0));
}

#[test]
fn test_read_multiple_words() {
    let mut text = Text::new(vec!["Hello World".to_string()]);
    let word = WordParser.read(&mut text).unwrap();

    assert_eq!(text.position, Position::new(5, 0));
    assert_eq!(word.content, "Hello");
}

#[test]
fn test_read_second_word() {
    let mut text = Text::new(vec!["Hello World".to_string()]);
    text.position = Position::new(5, 0);
    let word = WordParser.read(&mut text).unwrap();

    assert_eq!(text.position, Position::new(11, 0));
    assert_eq!(word.content, "World");
    assert_eq!(word.position, Position::new(6, 0));
}

#[test]
fn test_read_handles_leading_whitespace() {
    let mut text = Text::new(vec!["   Hello".to_string()]);
    let word = WordParser.read(&mut text).unwrap();

    assert_eq!(text.position, Position::new(8, 0));
    assert_eq!(word.content, "Hello");
    assert_eq!(word.position, Position::new(3, 0));
}

#[test]
fn test_read_empty_text() {
    let mut text = Text::new(vec!["".to_string()]);
    let result = WordParser.read(&mut text);

    assert!(result.is_err());
    assert_eq!(result.unwrap_err().typ, SxErrorType::ExpectedToken);
}
