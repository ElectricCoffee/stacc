use uuid::Uuid;
use token::Stack;

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