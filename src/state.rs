use crate::buildings::*;
use crate::conversion::Conversion;
use crate::regions::*;
use crate::resources::*;

#[derive(Debug)]
pub struct GameState<'a> {
    pub resources: ResourceTotal,
    pub regions: Vec<Region<'a>>,
}

impl<'a> GameState<'a> {
    pub fn init() -> GameState<'a> {
        GameState {
            resources: ResourceTotal::init(),
            regions: vec![
                Region::init_with_buildings(
                    "Lusitania",
                    vec![Building::init("Gathering Camp", vec![], vec![]), Building::init("Camp", vec![], vec![])],
                ),
                Region::init("Illyricum"),
            ],
        }
    }

    pub fn process_tick(&mut self) {
        for r in &mut self.regions {
            r.process_tick(&mut self.resources);
        }
    }

    pub fn buildings(&self) -> Vec<&Building<'a>> {
        self.regions.iter().flat_map(|x| &x.buildings).collect()
    }

    pub fn conversions(&self) -> Vec<&Conversion<'a>> {
        self.buildings().iter().flat_map(|x| &x.conversions).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_buildings_and_conversions() {
        let mut state = GameState::init();
        state.regions = vec![
            Region::init_with_buildings(
                "First Region",
                vec![
                    Building::init("First", vec![Conversion::init("First Convert", vec![], vec![])], vec![]),
                    Building::init(
                        "Second",
                        vec![
                            Conversion::init("Second Convert", vec![], vec![]),
                            Conversion::init("Third Convert", vec![], vec![]),
                        ],
                        vec![],
                    ),
                ],
            ),
            Region::init_with_buildings(
                "Second Region",
                vec![Building::init("Third", vec![Conversion::init("Fourth Convert", vec![], vec![])], vec![])],
            ),
        ];

        assert_eq!(2, state.regions.len());
        assert_eq!(3, state.buildings().len());
        assert_eq!(4, state.conversions().len());
    }
}
