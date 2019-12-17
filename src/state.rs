use std::ops::Index;

type ResourceQuantity = u64;

#[derive(Copy, Clone, PartialEq)]
pub enum Resources {
    Food,
    Fuel,

    // This must be incremented every time an item is added
    Size = 2,
}

pub struct ResourceAmount {
    pub resource: Resources,
    pub amount: ResourceQuantity,
}

const NUM_RESOURCES: usize = Resources::Size as usize;

pub struct Conversion<'a> {
    pub name: &'a str,
    pub input: [Option<ResourceAmount>; 4],
    pub output: [Option<ResourceAmount>; 4],
}

impl<'a> Conversion<'a> {
    pub fn has_input(&self, state: &GameState) -> bool {
        self.input
            .iter()
            .filter_map(|e| e.as_ref())
            .all(|x| state.has(x.resource, x.amount))
    }
}

pub struct GameState<'a> {
    pub resources: [ResourceQuantity; NUM_RESOURCES],
    pub conversions: Vec<Conversion<'a>>,
}

impl<'a> GameState<'a> {
    pub fn init() -> GameState<'a> {
        GameState {
            resources: [10; NUM_RESOURCES],
            conversions: vec![Conversion {
                name: "Convert",
                input: [
                    Some(ResourceAmount {
                        resource: Resources::Food,
                        amount: 4,
                    }),
                    None,
                    None,
                    None,
                ],
                output: Default::default(),
            }],
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
