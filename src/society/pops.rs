use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;

use super::prelude::EasyConstants;

#[derive(Component, Serialize, Deserialize, Clone, Default)]
pub struct PopComponent {
    pub job: Option<String>,
    pub happiness: f64,
    pub health: f64,
}

impl PopComponent {
    pub fn new() -> PopComponent {
        PopComponent {
            job: None,
            happiness: 50.0,
            health: 50.0,
        }
    }
}

pub fn tick_pop_stat_decay(ecs: &mut World) {
    let decay_rate = ecs.get_float_constant("DECAY_RATE");
    let mut pops = ecs.write_storage::<PopComponent>();
    for pop in (&mut pops).join() {
        pop.happiness = decay(pop.happiness, decay_rate);
        pop.health = decay(pop.health, decay_rate);
    }
}

fn decay(n: f64, rate: f64) -> f64 {
    let delta = (n - 50.0) / rate;
    n - delta
}

#[cfg(test)]
mod tests {
    use super::super::prelude::*;
    use super::*;
    use serde_json::{Number, Value};

    #[test]
    fn decay_up_below_midline() {
        let x = decay(40.0, 40.0);
        assert!(x > 40.0);
        assert!(x < 41.0);
    }

    #[test]
    fn decay_down_above_midline() {
        let x = decay(60.0, 40.0);
        assert!(x < 60.0);
        assert!(x > 59.0);
    }

    #[test]
    fn pops_stats_decay() {
        let mut ecs = register_world();
        ecs.write_resource::<ConstantLibrary>()
            .set("DECAY_RATE", Value::Number(Number::from_f64(40.0).unwrap()));

        {
            let id = ecs.next_id();
            let mut pop = PopComponent::new();
            pop.health = 20.0;
            pop.happiness = 80.0;
            ecs.create_entity().with(pop).with(id).build();
        }

        tick_pop_stat_decay(&mut ecs);
        let pop = ecs.read_storage::<PopComponent>().join().next().unwrap().clone();
        assert!(pop.happiness < 80.0);
        assert!(pop.happiness > 79.0);
        assert!(pop.health < 21.0);
        assert!(pop.health > 20.0);
    }
}
