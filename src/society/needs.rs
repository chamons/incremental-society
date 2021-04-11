use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::prelude::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct PopNeed {
    pub age: String,
    pub resources: HashMap<String, i32>,
}

impl PopNeed {
    #[cfg(test)]
    pub fn new_single(age: &str, resource: &str, amount: i32) -> PopNeed {
        PopNeed {
            age: age.to_string(),
            resources: [(resource.to_string(), amount)].iter().cloned().collect(),
        }
    }
}

pub struct PopNeedLibrary {
    needs: HashMap<String, PopNeed>,
}

impl PopNeedLibrary {
    pub fn load() -> PopNeedLibrary {
        let input = read_string("data", "needs.json");

        let needs: Vec<PopNeed> = serde_json::from_str(&input).unwrap();
        let needs: HashMap<String, PopNeed> = needs.iter().map(|j| (j.age.to_owned(), j.clone())).collect();

        PopNeedLibrary { needs }
    }

    pub fn get(&self, age: &str) -> &PopNeed {
        self.needs.get(age).expect(&format!("Unable to find age {}", age))
    }

    #[cfg(test)]
    pub fn add_need(&mut self, need: PopNeed) {
        self.needs.insert(need.age.to_owned(), need);
    }
}

pub fn tick_needs(ecs: &mut World) {
    let need_library = ecs.write_resource::<PopNeedLibrary>();
    let pop_need = need_library.get(&ecs.current_age());

    let mut resources = ecs.write_resource::<Resources>();
    let mut pops = ecs.write_storage::<PopComponent>();
    for pop in (&mut pops).join() {
        if !has_all_resources(&resources, &pop_need.resources) {
            pop.happiness -= ecs.get_float_constant("HAPPINESS_DROP_NEEDS_UNMET");
            // Consume whatever we can after happiness penalty
        }
        for (resource, &amount) in &pop_need.resources {
            resources.remove(resource, amount as u32);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_need_world() -> World {
        let mut ecs = register_world();
        ecs.write_resource::<PopNeedLibrary>().add_need(PopNeed {
            age: "TestAge".to_string(),
            resources: [("Food".to_string(), 2), ("Wood".to_string(), 1)].iter().cloned().collect(),
        });

        ecs.insert(Age::new("TestAge"));
        ecs
    }

    #[test]
    fn tick_fulfilled_needs() {
        let mut ecs = setup_need_world();
        {
            let mut resources = ecs.write_resource::<Resources>();
            resources.add("Food", 20);
            resources.add("Wood", 20);
        }

        for _ in 0..3 {
            let id = ecs.next_id();
            ecs.create_entity().with(PopComponent::new()).with(id).build();
        }
        tick_needs(&mut ecs);

        assert_eq!(14, ecs.read_resource::<Resources>().get("Food"));
        assert_eq!(17, ecs.read_resource::<Resources>().get("Wood"));
    }

    #[test]
    fn tick_unfulfilled_needs_reduces_happiness() {
        let mut ecs = setup_need_world();
        {
            let mut resources = ecs.write_resource::<Resources>();
            resources.add("Food", 20);
            resources.add("Wood", 2);
        }

        for _ in 0..3 {
            let id = ecs.next_id();
            ecs.create_entity().with(PopComponent::new()).with(id).build();
        }
        let before = calculate_average_happiness(&ecs);
        tick_needs(&mut ecs);
        let after = calculate_average_happiness(&ecs);

        assert_eq!(14, ecs.read_resource::<Resources>().get("Food"));
        assert_eq!(0, ecs.read_resource::<Resources>().get("Wood"));
        assert!(after < before);
    }
}
