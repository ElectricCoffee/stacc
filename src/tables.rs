use std::collections::HashMap;
use uuid::Uuid;
use token::{Token};
use error::{Result, Error};
use scope::{self, Scope};

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

/// Gets the parent ID of the current symbol table if it exists
pub fn get_parent_id(table: &SymbolTable) -> Option<Uuid> {
    if let Some(Token::Id(id)) = table.get(scope::PARENT) {
        Some(*id)
    } else {
        None
    }
}

/// Looks up a symbol in the scope table, using that particular scope's ID
/// If the symbol is found, the scope ID is returned.
/// If the symbol isn't found, and a parent is present, the parent ID is returned.
/// If the symbol isn't found, and there is no parent; then the symbol doesn't exist.
pub fn lookup(table: &mut ScopeTable, id: Uuid, symbol: &str) -> Lookup {
    use tables::Lookup::*;

    let symbol_table = table
        .get_mut(&id)
        .unwrap_or_else(|| Scope::invalid_id_panic(id));

    if symbol_table.contains_key(symbol) {
        Found(id)
    } else if let Some(parent_id) = get_parent_id(symbol_table) {
        CheckParent(parent_id)
    } else {
        NotFound
    }
}

/// Tries to find the symbol table in which a given symbol is located, regardless of depth.
/// If successful, it returns a mutable reference to the given symbol table, if not it returns `Error::UnknownIdentifier`
pub fn find_symbol<'a>(table: &'a mut ScopeTable, id: Uuid, symbol: &str) -> Result<&'a mut SymbolTable> {
    use tables::Lookup::*;

    match lookup(table, id, symbol) {
        Found(id) => Ok(table.get_mut(&id).unwrap()),
        CheckParent(id) => find_symbol(table, id, symbol),
        NotFound => Err(Error::UnknownIdentifier),
    }
}