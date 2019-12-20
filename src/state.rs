use crate::conversion::*;
use crate::resources::*;

#[derive(Debug)]
pub struct GameState<'a> {
    pub resources: ResourceTotal,
    pub conversions: Vec<Conversion<'a>>,
}

impl<'a> GameState<'a> {
    pub fn init() -> GameState<'a> {
        GameState {
            resources: ResourceTotal::init(),
            conversions: vec![],
        }
    }
}
