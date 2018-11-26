use parser;
use token::{Token, Stack};
use error::Error;
use tables;
use scope;

fn mk_num_stack() -> Stack {
    vec![Token::Number(12.0), Token::Number(2.0)]
}

fn mk_if_stack(condition: bool) -> Stack {
    vec![Token::String("Then case".into()), Token::String("Else case".into()), Token::Boolean(condition)]
}

/// Initialises the environment with an initial stack frame and scope and scope table.
/// Returns a pair containing a scope table and a stack-frame (i.e. vector of scopes)
fn initialise_env(initial_stack: Stack) -> (tables::ScopeTable, scope::StackFrames) {
    let mut scopes = tables::ScopeTable::new();
    let mut main_scope = scope::Scope::new(&mut scopes, None);
    main_scope.stack = initial_stack;
    let stack_frames = vec![main_scope];
    (scopes, stack_frames)
}

#[test]
fn test_add() {
    let (mut scopes, mut stack_frames) = initialise_env(mk_num_stack());

    let res = parser::parse_symbol(&mut scopes, &mut stack_frames, "+");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::Number(14.0)];

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack_frames[0].stack);
}
#[test]
fn test_sub() {
    let (mut scopes, mut stack_frames) = initialise_env(mk_num_stack());

    let res = parser::parse_symbol(&mut scopes, &mut stack_frames, "-");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::Number(10.0)];

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack_frames[0].stack);
}
#[test]
fn test_div() {
    let (mut scopes, mut stack_frames) = initialise_env(mk_num_stack());

    let res = parser::parse_symbol(&mut scopes, &mut stack_frames, "/");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::Number(6.0)];

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack_frames[0].stack);
}

#[test]
// true case
fn test_if_then() {
    let (mut scopes, mut stack_frames) = initialise_env(mk_if_stack(true));

    let res = parser::parse_symbol(&mut scopes, &mut stack_frames, "if");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::String("Then case".into())];

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack_frames[0].stack);
}

#[test]

// false case
fn test_if_else() {
    let (mut scopes, mut stack_frames) = initialise_env(mk_if_stack(false));
    let res = parser::parse_symbol(&mut scopes, &mut stack_frames, "if");
    let expected_res = Ok(());
    let expected_stack: Stack = vec![Token::String("Else case".into())];

    assert_eq!(expected_res, res);
    assert_eq!(expected_stack, stack_frames[0].stack);
}
#[test]
fn test_if_error() {
    let init = vec![Token::String("Then case".into()), Token::String("Else case".into()), Token::Number(42.0)];
    let (mut scopes, mut stack_frames) = initialise_env(init);

    let res = parser::parse_symbol(&mut scopes, &mut stack_frames, "if");
    let expected_res = Err(Error::ArgumentMismatch);

    assert_eq!(expected_res, res);
}