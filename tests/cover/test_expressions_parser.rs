use sx2rust::enums::ExpType;
use sx2rust::parsers::expressions_parsers::ExpressionParser;
use sx2rust::parsers::SxParser;
use sx2rust::text::Text;

#[test]
fn test_read_simple_positive() {
    do_the_test(  &[
        (vec!["23".to_string()], ExpType::Int),
        (vec!["ahoj".to_string()], ExpType::Unknown),
    ]);
}

#[test]
fn test_read_binary_positive() {
    do_the_test(  &[
        (vec!["23+3".to_string()], ExpType::Int),
        (vec!["23+a".to_string()], ExpType::Int),
        (vec!["23*a".to_string()], ExpType::Int),
        (vec!["23/a".to_string()], ExpType::Int),
        (vec!["23**a".to_string()], ExpType::Int),
        (vec!["(23+a)".to_string()], ExpType::Int),
        (vec!["2>3".to_string()], ExpType::Comparison),
        (vec!["2<3".to_string()], ExpType::Comparison),
        (vec!["2>=df".to_string()], ExpType::Comparison),
        (vec!["2<=df".to_string()], ExpType::Comparison)
    ])
}

#[test]
fn test_read_complex_positive() {
    do_the_test(  &[
        (vec!["(23+a)-4".to_string()], ExpType::Int),
        (vec!["(3+a)-(4)".to_string()], ExpType::Int),
        (vec!["3+(a-(4))".to_string()], ExpType::Int),
        (vec!["((3+a)-(4))".to_string()], ExpType::Int),
        (vec!["(3<a) and (4 == 4)".to_string()], ExpType::Bool),
        (vec!["3+1<a or 4>=ahoj".to_string()], ExpType::Bool)
    ])
}

#[test]
fn test_read_multi_line_positive() {
    do_the_test(  &[
        (vec!["23+3+".to_string(), "4".to_string()], ExpType::Int),
        (vec!["(23+3".to_string(), "+4)".to_string()], ExpType::Int),
        (vec!["23+3".to_string(), "+4".to_string()], ExpType::Int)
    ])
}


fn do_the_test(cases: &[(Vec<String>, ExpType)])  {
    for (lines, expected_type) in cases {
        let mut text = Text::new(lines.to_vec());

        match ExpressionParser.read(&mut text) {
            Ok(result) =>  {
                assert_eq!(
                    *expected_type,
                    result.exp_type().expect("Expression type must be defined"),
                    "Failed for lines: {:?}",
                    lines
                );
            }
            Err(e) => {
                panic!("Unexpected exception: {:?}\nText: {}", e, text.line());
            }
        }
    }
}



// fn do_the_test(cases: &[(Vec<&str>, ExpType)])  {
//     // fn assert_expression_types(cases: &[(Vec<&str>, ExpType)]) {
//     for (lines, expected_type) in cases {
//         assert_expression_type(lines, *expected_type);
//     }
// }
//
// fn assert_expression_type(lines: &[&str], expected_type: ExpType) {
//     let owned_lines = lines.iter().map(|line| (*line).to_string()).collect();
//     let mut text = Text::new(owned_lines);
//
//     let expression = ExpressionParser
//         .read(&mut text)
//         .unwrap_or_else(|error| panic!("Unexpected exception: {:?}\nText: {}", error, text.line()));
//
//     let actual_type = expression
//         .exp_type()
//         .expect("Expression type must be defined");
//
//     assert_eq!(
//         expected_type,
//         actual_type,
//         "Failed for lines: {:?}",
//         lines
//     );
// }



// private fun doTheTest(cases: List<Pair<List<String>, ExpType>>) {
// for ((lines, expectedType) in cases) {
// val text = Text(lines)
// try {
// val result = ExpressionParser.i().read(text)
// Assertions.assertEquals(expectedType, result.expType, "Failed for lines: $lines")
// } catch (e: SxError) {
// Assertions.fail("Unexpected exception: $e\nText: ${text.line}")
// }
// }
// }
//
// private fun doTheTestNegative(cases: List<Pair<List<String>, SxErrorType>>) {
// for ((lines, expectedType) in cases) {
// val text = Text(lines)
// val exception = assertThrows<SxError>("Expected SxError for lines: $lines") {
// ExpressionParser.i().read(text)
// }
// Assertions.assertEquals(expectedType, exception.typ, "Wrong error type for lines: $lines")
// }
// }