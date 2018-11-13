use uuid::Uuid;
use tables::{ScopeTable, SymbolTable};
use token::{Token, Stack};

pub const PARENT: &'static str = "$$PARENT$$";

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    parent: Option<Uuid>,
    id: Uuid,
    pub stack: Stack,
}

impl Scope {
    /// Creates a new scope and assigns it to the scope table
    pub fn new(scopes: &mut ScopeTable, parent: Option<Uuid>) -> Scope {
        let mut id = Uuid::new_v4();

        // if the newly created scope's ID is a name clash, try again
        if scopes.get(&id).is_some() {
            id = Uuid::new_v4();
        }

        let mut table_entry = SymbolTable::new();

        // if the scope has a parent, add special $$PARENT$$ entry 
        // so the parent can be referenced from the symbol table
        if let Some(parent_id) = parent {
            table_entry.insert(PARENT.into(), Token::Id(parent_id));
        }

        scopes.insert(id, table_entry);

        Scope {
            parent,
            id: id,
            stack: Stack::new(),
        }
    }

    /// Checks if the current scope has a parent
    pub fn has_parent(&self) -> bool {
        self.parent.is_some()
    }

    /// Gets the parent ID of the current scope if available
    pub fn parent_id(&self) -> Option<Uuid> {
        self.parent
    }

    /// Gets the ID of the current scope
    pub fn id(&self) -> Uuid {
        self.id
    }
}