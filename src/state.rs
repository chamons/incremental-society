use std::ops::{Index, IndexMut};

type ResourceQuantity = u64;

#[derive(Debug, Copy, Clone)]
pub enum Resources {
    Food,
    Fuel,

    // This must be incremented every time an item is added
    Size = 2,
}

#[derive(Debug, Copy, Clone)]
pub struct ResourceAmount {
    pub resource: Resources,
    pub amount: ResourceQuantity,
}

impl ResourceAmount {
    pub fn init(resource: Resources, amount: ResourceQuantity) -> ResourceAmount {
        ResourceAmount { resource, amount }
    }
}

const NUM_RESOURCES: usize = Resources::Size as usize;

#[derive(Debug)]
pub struct Conversion<'a> {
    pub name: &'a str,
    pub input: Vec<ResourceAmount>,
    pub output: Vec<ResourceAmount>,
}

impl<'a> Conversion<'a> {
    pub fn init_single(
        name: &'a str,
        input: ResourceAmount,
        output: ResourceAmount,
    ) -> Conversion<'a> {
        Conversion {
            name: name,
            input: vec![input],
            output: vec![output],
        }
    }

    pub fn init(
        name: &'a str,
        input: Vec<ResourceAmount>,
        output: Vec<ResourceAmount>,
    ) -> Conversion<'a> {
        Conversion {
            name: name,
            input: input,
            output: output,
        }
    }
}

impl<'a> Conversion<'a> {
    pub fn has_input(&self, state: &GameState) -> bool {
        self.input.iter().all(|x| state.has(x.resource, x.amount))
    }
}

#[derive(Debug)]
pub struct GameState<'a> {
    pub resources: [ResourceQuantity; NUM_RESOURCES],
    pub conversions: Vec<Conversion<'a>>,
}

impl<'a> GameState<'a> {
    pub fn init() -> GameState<'a> {
        GameState {
            resources: [0; NUM_RESOURCES],
            conversions: vec![],
        }
    }

    pub fn has(&self, resource: Resources, amount: ResourceQuantity) -> bool {
        self[resource] >= amount
    }
}

impl<'a> Index<Resources> for GameState<'a> {
    type Output = ResourceQuantity;

    fn index(&self, resource: Resources) -> &ResourceQuantity {
        &self.resources[resource as usize]
    }
}

impl<'a> IndexMut<Resources> for GameState<'a> {
    fn index_mut(&mut self, resource: Resources) -> &mut ResourceQuantity {
        &mut self.resources[resource as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gamestate_has_enough() {
        let mut state = GameState::init();
        state[Resources::Fuel] = 5;
        assert_eq!(true, state.has(Resources::Fuel, 5));
        assert_eq!(false, state.has(Resources::Fuel, 15));
        assert_eq!(false, state.has(Resources::Food, 1));
    }
}
