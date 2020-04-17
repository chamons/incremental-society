use std::collections::HashSet;

use super::{check_available, Conversion, GameState};

#[derive(Debug, Clone)]
pub struct Edict {
    pub name: String,
    pub conversion: Conversion,
    pub research: HashSet<String>,
    // 1 (default) means 1.0-1.0 range, always conversion amount
    // 2 means 1/2 to 2 range
    // 5 means 1/5 to 5 range
    // Amounts always rounded up
    pub effective_range: u32,
}

impl Edict {
    pub fn init(name: &str, conversion: Conversion) -> Edict {
        Edict::init_with_range(name, conversion)
    }

    pub fn init_with_range(name: &str, conversion: Conversion) -> Edict {
        Edict {
            name: name.to_owned(),
            conversion,
            research: HashSet::new(),
            effective_range: 1,
        }
    }

    pub fn with_research(mut self, research: Vec<&str>) -> Edict {
        self.research = research.iter().map(|x| (*x).to_owned()).collect();
        self
    }

    pub fn with_effective_range(mut self, effective_range: u32) -> Edict {
        self.effective_range = effective_range;
        self
    }

    pub fn is_available(&self, state: &GameState) -> bool {
        check_available(&self.research, &state)
    }
}
