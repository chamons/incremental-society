use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;

pub type Identifier = u64;

pub struct IdentifierLibrary {
    pub next: Identifier,
}

impl IdentifierLibrary {
    pub fn load() -> IdentifierLibrary {
        IdentifierLibrary { next: 0 }
    }

    pub fn next(&mut self) -> IdentifierComponent {
        let next = self.next;
        self.next += 1;
        IdentifierComponent::new(next)
    }
}

#[derive(Component, Serialize, Deserialize, Clone)]
pub struct IdentifierComponent {
    pub id: Identifier,
}

impl IdentifierComponent {
    pub fn new(id: Identifier) -> IdentifierComponent {
        IdentifierComponent { id }
    }
}

pub trait EasyId {
    fn get_id(&self, entity: Entity) -> Option<Identifier>;
    fn next_id(&mut self) -> IdentifierComponent;
}

impl EasyId for World {
    fn get_id(&self, entity: Entity) -> Option<Identifier> {
        self.read_storage::<IdentifierComponent>().get(entity).map(|i| i.id)
    }

    fn next_id(&mut self) -> IdentifierComponent {
        self.write_resource::<IdentifierLibrary>().next()
    }
}
