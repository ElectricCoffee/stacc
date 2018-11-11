use std::collections::HashMap;
use uuid::Uuid;
use token::{Token};

/// Stores the name of a symbol and the associated data stored within
pub type SymbolTable = HashMap<String, Token>;

/// Stores the symbol table of each scope
pub type ScopeTable = HashMap<Uuid, SymbolTable>;