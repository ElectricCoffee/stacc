use std::collections::HashMap;
use uuid::Uuid;
use token::{Token, Stack};

/// Stores the name of a symbol and the associated data stored within
pub type SymbolTable = HashMap<String, Token>;

/// Stores the symbol table of each scope
pub type ScopeTable = HashMap<Uuid, SymbolTable>;

pub struct Scope {
    parent: Option<Uuid>,
    id: Uuid,
    stack: Stack,
}

impl Scope {
    fn new(parent: Option<Uuid>) -> Scope {
        Scope {
            parent,
            id: Uuid::new_v4(),
            stack: Stack::new(),
        }
    }

    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }
}