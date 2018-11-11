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
    fn new(scopes: &mut ScopeTable, parent: Option<Uuid>) -> Scope {
        let mut id = Uuid::new_v4();

        // if the newly created scope's ID is a name clash, try again
        if scopes.get(&id).is_some() {
            id = Uuid::new_v4();
        }

        scopes.insert(id, SymbolTable::new());

        Scope {
            parent,
            id: id,
            stack: Stack::new(),
        }
    }

    fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    fn id(&self) -> Uuid {
        self.id
    }
}