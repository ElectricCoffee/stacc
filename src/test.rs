use parser;
use token::{Token, Stack};
use error::Error;
use tables;
use scope;

fn mk_num_stack() -> Stack {
    vec![Token::Number(12.0), Token::Number(2.0)].into()
}

fn mk_if_stack(condition: bool) -> Stack {
    vec![Token::String("Then case".into()), Token::String("Else case".into()), Token::Boolean(condition)].into()
}

#[test]
fn test_binop() {
    let mut scopes = tables::ScopeTable::new();
    let mut main_scope = scope::Scope::new(&mut scopes, None);
    main_scope.stack = mk_num_stack();

    let res = parser::parse_symbol(&mut scopes, &mut main_scope, "+");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::Number(14.0)].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, main_scope.stack);

    main_scope.stack = mk_num_stack();
    let res = parser::parse_symbol(&mut scopes, &mut main_scope, "-");
    let expected_stack: Stack = vec![Token::Number(10.0)].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, main_scope.stack);

    main_scope.stack = mk_num_stack();
    let res = parser::parse_symbol(&mut scopes, &mut main_scope, "/");
    let expected_stack: Stack = vec![Token::Number(6.0)].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, main_scope.stack);
}

#[test]
fn test_if() {
    // setup
    let mut scopes = tables::ScopeTable::new();
    let mut main_scope = scope::Scope::new(&mut scopes, None);

    // true case
    main_scope.stack = mk_if_stack(true);
    let res = parser::parse_symbol(&mut scopes, &mut main_scope, "if");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::String("Then case".into())].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, main_scope.stack);

    // false case
    main_scope.stack = mk_if_stack(false);
    let res = parser::parse_symbol(&mut scopes, &mut main_scope, "if");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::String("Else case".into())].into();

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, main_scope.stack);

    // check error case
    main_scope.stack = vec![Token::String("Then case".into()), Token::String("Else case".into()), Token::Number(42.0)].into();
    let res = parser::parse_symbol(&mut scopes, &mut main_scope, "if");
    let expected_res = Err(Error::ArgumentMismatch);

    assert_eq!(expected_res, res);
}

#[test]
fn test_lookup() {
    // setup
    let mut scopes = tables::ScopeTable::new();
    let mut main_scope = scope::Scope::new(&mut scopes, None);

    main_scope.stack = Vec::new().into(); // doesn't need to contain anything
    let res = parser::parse_symbol(&mut scopes, &mut main_scope, "sdfjhsdjklfdskjhf"); // definitely invalid token
    let expected_res = Err(Error::UnknownIdentifier);

    assert_eq!(expected_res, res);
}