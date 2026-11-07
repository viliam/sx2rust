use sx2rust::enums::{ExpType, SxErrorType};
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
fn test_read_simple_negative() {
    do_the_test_negative(  &[
        (vec!["23dd".to_string()], SxErrorType::ExpectedInt),
        (vec!["ahoj+".to_string()], SxErrorType::UnexpectedPrefix),
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
fn test_read_complex_negative() {
    do_the_test_negative( &[
        (vec!["(23+a-4".to_string()], SxErrorType::EndOfFile),
        (vec!["(3+3a)-(4)".to_string()], SxErrorType::ExpectedInt),
        (vec![")3+(a-(4))".to_string()], SxErrorType::UnexpectedPrefix),
        (vec!["((3+>a)-(4))".to_string()], SxErrorType::UnexpectedPrefix),
        (vec!["(3<a) ad (4 == 4)".to_string()], SxErrorType::ExpectedOperator),
        (vec!["3+1<a( or 4>=ahoj".to_string()], SxErrorType::UncorrectedEndOfExpression)
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

#[test]
fn test_read_multi_line_negative() {
    do_the_test_negative( &[
        (vec!["23+3+".to_string(), "+4".to_string()], SxErrorType::UnexpectedPrefix),
    ])
}

#[test]
fn test_read_commands() {
    do_the_test(  &[
        (vec!["func(1, 2)".to_string()], ExpType::Unknown),
        (vec!["lolo(a+1,b,c)".to_string()], ExpType::Unknown),
        (vec!["func(1)(2)".to_string()], ExpType::Unknown),
        (vec!["arr[0](1)".to_string()], ExpType::Unknown),
    ])
}

#[test]
fn test_read_array() {
    do_the_test(  &[
        (vec!["[1, 2]".to_string()], ExpType::Unknown),
        (vec!["[]".to_string()], ExpType::Unknown),
        (vec!["arr[0]".to_string()], ExpType::Unknown),
        (vec!["arr[i+1]".to_string()], ExpType::Unknown),
        (vec!["arr[0][1]".to_string()], ExpType::Unknown),
        (vec!["func(1, 2)[0]".to_string()], ExpType::Unknown),
        (vec!["arr[0]+1".to_string()], ExpType::Int),
        (vec!["[1, 2][0]".to_string()], ExpType::Unknown),
    ])
}

#[test]
fn test_read_array_negative() {
    do_the_test_negative( &[
        (vec!["arr[0".to_string()], SxErrorType::UnexpectedPrefix),
        (vec!["[1, 2".to_string()], SxErrorType::UnexpectedPrefix),
        (vec!["arr[0 1]".to_string()], SxErrorType::UncorrectedEndOfExpression),
    ])
}

#[test]
fn test_read_member_access_positive() {
    do_the_test( &[
        (vec!["obj.name".to_string()], ExpType::Unknown),
        (vec!["a.b.c".to_string()], ExpType::Unknown),
        (vec!["a.b.c.d".to_string()], ExpType::Unknown),
        (vec!["obj.method(1)".to_string()], ExpType::Unknown),
        (vec!["obj.method(1, 2)".to_string()], ExpType::Unknown),
        (vec!["obj.arr[0]".to_string()], ExpType::Unknown),
        (vec!["arr[0].name".to_string()], ExpType::Unknown),
        (vec!["func(1).name".to_string()], ExpType::Unknown),
        (vec!["obj.a.b(1).c[0].d".to_string()], ExpType::Unknown),
        (vec!["obj.value+1".to_string()], ExpType::Int),
        (vec!["obj.value+other.value".to_string()], ExpType::Int),
        (vec!["obj.x>obj.y".to_string()], ExpType::Comparison),
        (vec!["(obj.name)".to_string()], ExpType::Unknown),
        (vec!["obj.a and obj.b".to_string()], ExpType::Bool),
    ])
}

#[test]
fn test_read_member_access_negative() {
    do_the_test_negative( &[
        (vec!["obj.".to_string()], SxErrorType::ExpectedToken),
        (vec!["obj.name.".to_string()], SxErrorType::ExpectedToken),
        (vec!["obj.+name".to_string()], SxErrorType::ExpectedToken),
    ])
}
//
// @Test
// fun testReadMemberAccessNegative() {
// doTheTestNegative(listOf(
// listOf("obj.") to SxErrorType.EXPECTED_TOKEN,
// listOf("obj.name.") to SxErrorType.EXPECTED_TOKEN,
// listOf("obj.+name") to SxErrorType.EXPECTED_TOKEN,
// ))
// }


//
// @Test
// fun testReadMemberAccessPositive() {
// doTheTest(listOf(
// listOf("obj.name") to ExpType.UNKNOWN,
// listOf("a.b.c") to ExpType.UNKNOWN,
// listOf("a.b.c.d") to ExpType.UNKNOWN,
// listOf("obj.method(1)") to ExpType.UNKNOWN,
// listOf("obj.method(1, 2)") to ExpType.UNKNOWN,
// listOf("obj.arr[0]") to ExpType.UNKNOWN,
// listOf("arr[0].name") to ExpType.UNKNOWN,
// listOf("func(1).name") to ExpType.UNKNOWN,
// listOf("obj.a.b(1).c[0].d") to ExpType.UNKNOWN,
// listOf("obj.value+1") to ExpType.INT,
// listOf("obj.value+other.value") to ExpType.INT,
// listOf("obj.x>obj.y") to ExpType.COMPARISON,
// listOf("(obj.name)") to ExpType.UNKNOWN,
// listOf("obj.a and obj.b") to ExpType.BOOL,
// ))
// }
//
// @Test
// fun testReadMemberAccessNegative() {
// doTheTestNegative(listOf(
// listOf("obj.") to SxErrorType.EXPECTED_TOKEN,
// listOf("obj.name.") to SxErrorType.EXPECTED_TOKEN,
// listOf("obj.+name") to SxErrorType.EXPECTED_TOKEN,
// ))
// }



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

fn do_the_test_negative(cases: &[(Vec<String>, SxErrorType)])  {
    for (lines, expected_type) in cases {
        let mut text = Text::new(lines.to_vec());
        let result = ExpressionParser.read(&mut text);

        assert!(
            result.is_err(),
            "Expected error for lines: {:?}, but parsing succeeded",
            lines
        );
        assert_eq!(
            result.unwrap_err().typ,
            *expected_type,
            "Wrong error type for lines: {:?}",
            lines
        );

    }
}

