use std::ops::Index;

#[derive(Copy, Clone, PartialEq)]
pub enum Resources {
    Food,
    Fuel,

    // This must be incremented every time an item is added
    Size = 2,
}

pub struct ResourceAmount {
    pub resource: Resources,
    pub amount: u32,
}

const NUM_RESOURCES: usize = Resources::Size as usize;

pub struct Conversion<'a> {
    pub name: &'a str,
    pub input: [Option<ResourceAmount>; 4],
    pub output: [Option<ResourceAmount>; 4],
}

pub struct GameState<'a> {
    pub resources: [u32; NUM_RESOURCES],
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

    pub fn has(&self, resource: Resources, amount: u32) -> bool {
        self[resource] >= amount
    }
}

impl<'a> Index<Resources> for GameState<'a> {
    type Output = u32;

    fn index(&self, resource: Resources) -> &u32 {
        &self.resources[resource as usize]
    }
}
