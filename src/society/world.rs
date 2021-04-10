use specs::prelude::*;

use super::prelude::*;

pub fn register_world() -> World {
    let mut ecs = World::new();
    ecs.register::<IdentifierComponent>();
    ecs.register::<PopComponent>();

    ecs.insert(ConstantLibrary::load());
    ecs.insert(IdentifierLibrary::load());
    ecs.insert(JobLibrary::load());

    ecs.insert(Resources::new());

    ecs
}

pub fn create_world() -> World {
    let mut ecs = register_world();

    for _ in 0..5 {
        let id = ecs.next_id();
        ecs.create_entity().with(PopComponent::new()).with(id).build();
    }

    ecs.write_resource::<Resources>().add("Food", 10);
    ecs.write_resource::<Resources>().add("Wood", 2);

    ecs
}

pub fn tick(ecs: &mut World) {
    tick_jobs(ecs);
}
