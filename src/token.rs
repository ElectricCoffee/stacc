use std::collections::VecDeque;
use error::*;
use scope::Scope;
use uuid::Uuid;

pub type Stack = VecDeque<Token>;

/// The tokens that make up the programming lanugage
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    /// Keeps track of numeric values (May need to be replaced by a custom value) 1, 2.0, -3, etc.
    Number(f64),
    /// Holds the names of symbols (subject to change) if, for, $a, $value, etc
    Symbol(String),
    /// Holds strings, "you get the idea"
    String(String),
    /// Used for nested scopes, (...)
    Scope(Scope),
    /// Used for lists, [...]
    List(Stack),
    /// Used for booleans, true and false
    Boolean(bool),
    /// Used to store the UUID of a scope
    Id(Uuid),
    /// Matches the ( token     
    BeginScope,
    /// Matches the ) token  
    EndScope,
    /// Matches the [ token
    BeginList,
    /// Matches the ] token
    EndList,
    /// Used for operations that don't produce any results
    Void,
}

impl Token {
    /// Attempts to get a token as a number.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_number(&self) -> Result<f64> {
        match self {
            Token::Number(n) => Ok(*n),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_number(&self) -> bool {
        match self {
            Token::Number(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a symbol string.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_symbol(&self) -> Result<String> {
        match self {
            Token::Symbol(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Token::Symbol(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a string.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_string(&self) -> Result<String> {
        match self {
            Token::String(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_string(&self) -> bool {
        match self {
            Token::String(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a scope.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_scope(&self) -> Result<Scope> {
        match self {
            Token::Scope(s) => Ok(s.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_scope(&self) -> bool {
        match self {
            Token::Scope(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a scope.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_list(&self) -> Result<Stack> {
        match self {
            Token::List(l) => Ok(l.clone()),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_list(&self) -> bool {
        match self {
            Token::List(_) => true,
            _ => false,
        }
    }

    /// Attempts to get a token as a boolean.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_boolean(&self) -> Result<bool> {
        match self {
            Token::Boolean(b) => Ok(*b),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_boolean(&self) -> bool {
        match self {
            Token::Boolean(_) => true,
            _ => false,
        }
    }

        /// Attempts to get a token as a boolean.
    /// If it fails, it returns an argument mismatch error.
    pub fn get_id(&self) -> Result<Uuid> {
        match self {
            Token::Id(i) => Ok(*i),
            _ => Err(Error::ArgumentMismatch),
        }
    }

    pub fn is_id(&self) -> bool {
        match self {
            Token::Id(_) => true,
            _ => false,
        }
    }

    pub fn is_void(&self) -> bool {
        match self {
            Token::Void => true,
            _ => false,
        }
    }
}