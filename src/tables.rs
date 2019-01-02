use std::collections::HashMap;
use uuid::Uuid;
use token::{Token};
use error::{Result, Error};
use scope::{self, Scope};

pub trait SymTable {
    fn get_parent_id(&self) -> Option<Uuid>;
}

pub trait FrameTable {
    fn lookup(&mut self, Uuid, &str) -> Lookup;
    fn find_symbol<'a>(&'a mut self, Uuid, &str) -> Result<&'a mut SymbolTable>;
}

/// Stores the name of a symbol and the associated data stored within
pub type SymbolTable = HashMap<String, Token>;

/// Stores the symbol table of each scope
pub type ScopeTable = HashMap<Uuid, SymbolTable>;
 
/// Handles the three different symbol table lookup cases
pub enum Lookup {
    /// A lookup was successful at the given ID
    Found(Uuid),
    /// The lookup failed, but the current scope has a parent with the given ID
    CheckParent(Uuid),
    /// The lookup failed, and the current scope has no parent.
    NotFound,
}

impl SymTable for SymbolTable {
    /// Gets the parent ID of the current symbol table if it exists
    fn get_parent_id(&self) -> Option<Uuid> {
        if let Some(Token::Id(id)) = self.get(scope::PARENT) {
            Some(*id)
        } else {
            None
        }
    }
}

impl FrameTable for ScopeTable {
    /// Looks up a symbol in the scope table, using that particular scope's ID
    /// If the symbol is found, the scope ID is returned.
    /// If the symbol isn't found, and a parent is present, the parent ID is returned.
    /// If the symbol isn't found, and there is no parent; then the symbol doesn't exist.
    fn lookup(&mut self, id: Uuid, symbol: &str) -> Lookup {
        use tables::Lookup::*;

        let symbol_table = self
            .get_mut(&id)
            .unwrap_or_else(|| Scope::invalid_id_panic(id));

        if symbol_table.contains_key(symbol) {
            Found(id)
        } else if let Some(parent_id) = symbol_table.get_parent_id() {
            CheckParent(parent_id)
        } else {
            NotFound
        }
    }

    /// Tries to find the symbol table in which a given symbol is located, regardless of depth.
    /// If successful, it returns a mutable reference to the given symbol table, if not it returns `Error::UnknownIdentifier`
    fn find_symbol<'a>(&'a mut self, id: Uuid, symbol: &str) -> Result<&'a mut SymbolTable> {
        use tables::Lookup::*;

        match self.lookup(id, symbol) {
            Found(id) => Ok(self.get_mut(&id).unwrap()),
            CheckParent(id) => self.find_symbol(id, symbol),
            NotFound => Err(Error::UnknownIdentifier),
        }
    }
}