use std::ops::Index;

pub enum Resources {
    Food,
    Fuel,

    // This must be incremented every time an item is added
    Size = 2,
}

const NUM_RESOURCES: usize = Resources::Size as usize;

pub struct GameState {
    resources: [u32; NUM_RESOURCES],
}

impl GameState {
    pub fn init() -> GameState {
        GameState {
            resources: [0; NUM_RESOURCES],
        }
    }
}

impl Index<Resources> for GameState {
    type Output = u32;

    fn index(&self, resource: Resources) -> &u32 {
        &self.resources[resource as usize]
    }
}

pub fn process_tick(state: &mut GameState) {
    println!("{}", state[Resources::Food]);
}
