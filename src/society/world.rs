use specs::prelude::*;

use super::prelude::*;

pub fn register_world() -> World {
    let mut ecs = World::new();
    ecs.register::<PopComponent>();

    ecs.insert(Resources::new());
    ecs.write_resource::<Resources>().add("Food", 10);
    ecs.write_resource::<Resources>().add("Wood", 2);

    ecs
}

pub fn create_world() -> World {
    let mut ecs = register_world();
    for _ in 0..5 {
        ecs.create_entity().with(PopComponent::default()).build();
    }
    ecs
}
