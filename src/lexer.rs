use regex::Regex;
use token::Token;

lazy_static!{
    pub static ref NUMBER: Regex      = Regex::new(r"-?(\d*\.\d+|\d+)").unwrap();
    pub static ref BOOLEAN: Regex     = Regex::new(r"(true|false)").unwrap();
    pub static ref SYMBOL: Regex      = Regex::new(r"[$a-zA-Z][$\w]*").unwrap();
    pub static ref BEGIN_SCOPE: Regex = Regex::new(r"\(").unwrap();
    pub static ref END_SCOPE: Regex   = Regex::new(r"\)").unwrap();
    pub static ref BEGIN_LIST: Regex  = Regex::new(r"\[").unwrap();
    pub static ref END_LIST: Regex    = Regex::new(r"\]").unwrap();
}

/// Takes the input string and returns a vec of tokens.
pub fn lex(input: &str) -> Vec<Token> {
    unimplemented!();
}

// Checks if the string mathes the expression and wraps the string in an Option
pub fn get_match<'a>(regex: &Regex, str: &'a str) -> Option<&'a str> {
    if regex.is_match(str) {
        Some(str)
    } else {
        None
    }
}

/// Takes a string reference and turns it into a bool token
/// It does so via its corresponding regular expression.
pub fn mk_number(str: &str) -> Option<Token> {
    get_match(&NUMBER, str)
        .and_then(|n| n.parse::<f64>().ok())
        .map(Token::Number)
}

/// Takes a string reference and turns it into a bool token
/// It does so via its corresponding regular expression.
pub fn mk_bool(str: &str) -> Option<Token> {
    get_match(&BOOLEAN, str)
        .and_then(|b| b.parse::<bool>().ok())
        .map(Token::Boolean)
}

/// Takes a string reference and turns it into a symbol token
/// It does so via its corresponding regular expression.
pub fn mk_symbol(str: &str) -> Option<Token> {
    get_match(&SYMBOL, str)
        .map(|sym| sym.to_owned())
        .map(Token::Symbol)
}

/// Takes a string reference and turns it into a begin-scope token
/// It does so via its corresponding regular expression.
pub fn mk_begin_scope(str: &str) -> Option<Token> {
    get_match(&BEGIN_SCOPE, str)
        .map(|_| Token::BeginScope)
}

/// Takes a string reference and turns it into a end-scope token
/// It does so via its corresponding regular expression.
pub fn mk_end_scope(str: &str) -> Option<Token> {
    get_match(&END_SCOPE, str)
        .map(|_| Token::EndScope)
}

/// Takes a string reference and turns it into a begin-list token
/// It does so via its corresponding regular expression.
pub fn mk_begin_list(str: &str) -> Option<Token> {
    get_match(&BEGIN_LIST, str)
        .map(|_| Token::BeginList)
}

/// Takes a string reference and turns it into a end-list token
/// It does so via its corresponding regular expression.
pub fn mk_end_list(str: &str) -> Option<Token> {
    get_match(&END_LIST, str)
        .map(|_| Token::EndList)
}