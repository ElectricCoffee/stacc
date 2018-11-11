use parser;
use token::{Token, Stack};

fn mk_num_stack() -> Stack {
    vec![Token::Number(12.0), Token::Number(2.0)].into()
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