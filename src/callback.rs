use token::Token;
use error::{Error, Result};
use scope::Scope;
use tables::ScopeTable;

/// Handles encoding callback information, with the arity size of the function that it expects
pub struct Callback {
    /// Encodes the callback's arity -- its number of arguments.
    pub arity: usize,
    /// Encodes the built-in function type. 
    /// Designed to be as generic as possible, to accomodate the widest range of functions. 
    pub func: fn(&mut ScopeTable, &mut Scope, &[Token]) -> Result<Token>,
}

impl Callback {
    /// Invokes the callback's internal function.
    /// Returns an Error::ArityMismatch if the expected arity doesn't match the number of args given.
    /// Otherwise it returns whatever the inner function returns.
    pub fn invoke(&self, table: &mut ScopeTable, scope: &mut Scope, args: &[Token]) -> Result<Token> {
        if self.arity != args.len() {
            return Err(Error::ArityMismatch);
        }

        let fun = self.func;
        fun(table, scope, args)
    }
}