use sx2rust::enums::SxErrorType;
use sx2rust::position::Position;
use sx2rust::text::Text;

#[test]
fn test_next_char_position_simple_text() {
    let mut text = Text::new(vec!["Hello".to_string(), "World".to_string()]);
    let result = text.next_char_position();
    assert!(result.is_some());
    let pos = result.unwrap();
    assert_eq!(pos.x, 0);
    assert_eq!(pos.y, 0);
}

#[test]
fn test_next_char_position_empty_text() {
    let mut text = Text::new(vec![]);
    let result = text.next_char_position();
    assert!(result.is_none());
}

#[test]
fn test_next_char_position_whitespace_text() {
    let mut text = Text::new(vec![
        "   ".to_string(),
        "  ".to_string(),
        "Hello".to_string(),
    ]);
    let result = text.next_char_position();
    assert!(result.is_some());
    let pos = result.unwrap();
    assert_eq!(pos.x, 0);
    assert_eq!(pos.y, 2);
}

#[test]
fn test_next_char_simple_text() {
    let mut text = Text::new(vec!["    Hello".to_string(), "World".to_string()]);
    let result = text.next_char();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 'H');
    assert_eq!(text.position.x, 0);
    assert_eq!(text.position.y, 0);
}

#[test]
fn test_next_char_empty_text() {
    let mut text = Text::new(vec![]);
    let result = text.next_char();
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().typ, SxErrorType::EndOfFile);
}

#[test]
fn test_look_ahead_finds_word() {
    let text = Text::new(vec!["TestWord example".to_string()]);
    let result = text.peek_word();
    assert_eq!(result, "TestWord");
}

#[test]
fn test_look_ahead_empty_string() {
    let text = Text::new(vec!["".to_string()]);
    let result = text.peek_word();
    assert_eq!(result, "");
}

#[test]
fn test_look_ahead_start_index_beyond_length() {
    let mut text = Text::new(vec!["ShortWord".to_string()]);
    text.position = Position::new(9, 0);
    let result = text.peek_word();
    assert_eq!(result, "");
}

#[test]
fn test_look_ahead_non_word_character() {
    let mut text = Text::new(vec!["Test1@Word".to_string()]);
    text.position = Position::new(5, 0);
    let result = text.peek_word();
    assert_eq!(result, "");
}
