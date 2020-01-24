use super::{check_available, Conversion, GameState};
use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct Edict {
    pub name: String,
    pub conversion: Conversion,
    pub research: HashSet<String>,
}

impl Edict {
    pub fn init(name: &str, conversion: Conversion) -> Edict {
        Edict {
            name: name.to_owned(),
            conversion,
            research: HashSet::new(),
        }
    }

    pub fn with_research(mut self, research: Vec<&str>) -> Edict {
        self.research = research.iter().map(|x| (*x).to_owned()).collect();
        self
    }

    pub fn is_available(&self, state: &GameState) -> bool {
        check_available(&self.research, &state)
    }
}
