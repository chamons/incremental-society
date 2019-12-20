use crate::buildings::*;
use crate::conversion::Conversion;
use crate::resources::*;

#[derive(Debug)]
pub struct GameState<'a> {
    pub resources: ResourceTotal,
    pub buildings: Vec<Building<'a>>,
}

impl<'a> GameState<'a> {
    pub fn init() -> GameState<'a> {
        GameState {
            resources: ResourceTotal::init(),
            buildings: vec![],
        }
    }

    pub fn conversions(&self) -> Vec<&Conversion<'a>> {
        self.buildings.iter().flat_map(|x| &x.conversions).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversions_all_buildings() {
        let mut state = GameState::init();
        state.buildings = vec![
            Building::init(
                "First",
                vec![Conversion::init("First Convert", vec![], vec![])],
            ),
            Building::init(
                "Second",
                vec![
                    Conversion::init("Second Convert", vec![], vec![]),
                    Conversion::init("Third Convert", vec![], vec![]),
                ],
            ),
        ];

        assert_eq!(3, state.conversions().len());
    }
}
