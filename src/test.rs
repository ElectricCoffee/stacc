use parser;
use token::{Token, Stack};
use error::Error;

fn mk_num_stack() -> Stack {
    vec![Token::Number(12.0), Token::Number(2.0)].into()
}

fn mk_if_stack(condition: bool) -> Stack {
    vec![Token::String("Then case".into()), Token::String("Else case".into()), Token::Boolean(condition)].into()
}

#[test]
fn test_binop() {
    let mut stack: Stack = mk_num_stack();
    let res = parser::parse_nary(&mut stack, "+");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::Number(14.0)].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack);

    let mut stack: Stack = mk_num_stack();
    let res = parser::parse_nary(&mut stack, "-");
    let expected_stack: Stack = vec![Token::Number(10.0)].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack);

    let mut stack: Stack = mk_num_stack();
    let res = parser::parse_nary(&mut stack, "/");
    let expected_stack: Stack = vec![Token::Number(6.0)].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack);
}

#[test]
fn test_if() {
    let mut stack = mk_if_stack(true);
    let res = parser::parse_nary(&mut stack, "if");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::String("Then case".into())].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack);

    let mut stack = mk_if_stack(false);
    let res = parser::parse_nary(&mut stack, "if");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::String("Else case".into())].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack);

    // check error case
    let mut stack: Stack = vec![Token::String("Then case".into()), Token::String("Else case".into()), Token::Number(42.0)].into();
    let res = parser::parse_nary(&mut stack, "if");
    let expected_res = Err(Error::ArgumentMismatch);

    assert_eq!(expected_res, res);
}

#[test]
fn test_lookup() {
    let mut stack: Stack = Vec::new().into(); // doesn't need to contain anything
    let res = parser::parse_nary(&mut stack, "sdfjhsdjklfdskjhf"); // definitely invalid token
    let expected_res = Err(Error::UnknownIdentifier);

    assert_eq!(expected_res, res);
}