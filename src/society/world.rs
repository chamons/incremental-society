use specs::prelude::*;

use super::prelude::*;

pub struct Age {
    pub current: String,
}

impl Age {
    pub fn new(current: &str) -> Age {
        Age { current: current.to_string() }
    }
}

pub struct Time {
    pub tick: u64,
}

impl Time {
    pub fn new() -> Time {
        Time { tick: 0 }
    }

    pub fn tick(&mut self) -> u64 {
        self.tick += 1;
        self.tick - 1
    }
}

pub trait EasyAge {
    fn current_age(&self) -> String;
}

impl EasyAge for World {
    fn current_age(&self) -> String {
        self.read_resource::<Age>().current.to_string()
    }
}

pub fn register_world() -> World {
    let mut ecs = World::new();
    ecs.register::<IdentifierComponent>();
    ecs.register::<PopComponent>();

    ecs.insert(ConstantLibrary::load());
    ecs.insert(IdentifierLibrary::load());
    ecs.insert(JobLibrary::load());
    ecs.insert(PopNeedLibrary::load());
    ecs.insert(Time::new());

    ecs.insert(Resources::new());

    ecs
}

pub fn create_world() -> World {
    let mut ecs = register_world();

    ecs.insert(Age::new(&ecs.get_string_constant("STARTING_AGE")));

    for _ in 0..5 {
        let id = ecs.next_id();
        ecs.create_entity().with(PopComponent::new()).with(id).build();
    }

    ecs.write_resource::<Resources>().add("Food", 10);
    ecs.write_resource::<Resources>().add("Wood", 2);

    ecs
}

pub fn tick(ecs: &mut World) {
    let tick = ecs.write_resource::<Time>().tick();

    if tick % ecs.get_constant("STAT_DECAY_TIME") as u64 == 0 {
        tick_pop_stat_decay(ecs);
    }

    if tick % ecs.get_constant("JOB_TIME") as u64 == 0 {
        tick_jobs(ecs);
    }
    if tick % ecs.get_constant("NEED_TIME") as u64 == 0 {
        tick_needs(ecs);
    }
    if tick % 1000 == 0 {
        println!("Tick");
    }
}
