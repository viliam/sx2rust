use sx2rust::enums::{ExpType, SxErrorType};
use sx2rust::parsers::expressions_parsers::{IntegerParser, SimpleExpressionParser};
use sx2rust::parsers::SxParser;
use sx2rust::text::Text;

#[test]
fn test_read_valid_integer() {
    let mut text = Text::new(vec!["123".to_string()]);
    let result = SimpleExpressionParser.read(&mut text).unwrap();
    assert_eq!(ExpType::Int, result.exp_type().unwrap());
}

#[test]
fn test_read_invalid_integer_raises_error() {
    let mut text = Text::new(vec!["abc".to_string()]);

    let result = IntegerParser.read(&mut text);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().typ, SxErrorType::ExpectedInt);
}

#[test]
fn test_read_simple_expression() {
    let mut text1 = Text::new(vec!["23".to_string()]);
    let result1 = SimpleExpressionParser.read(&mut text1).unwrap();
    assert_eq!(ExpType::Int, result1.exp_type().unwrap());

    let mut text2 = Text::new(vec!["ahoj".to_string()]);
    let result2 = SimpleExpressionParser.read(&mut text2).unwrap();
    assert_eq!(ExpType::Unknown, result2.exp_type().unwrap());
}
