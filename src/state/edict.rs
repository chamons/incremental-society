use std::collections::HashSet;

use super::{check_available_by_research, Conversion, GameState};

#[derive(Debug, Clone)]
pub struct Edict {
    pub name: String,
    pub conversion: Conversion,
    pub research: HashSet<String>,
    // Amount added to output (only) of edict conversion
    // 1 (default) means 1.0-1.0 range, always conversion amount
    // 2 means 1/2 to 2 range
    // 5 means 1/5 to 5 range
    // Amounts always rounded up
    pub effective_range: u32,
    // An amount added to effective_range before being applied
    pub effective_bonus: u32,
}

impl Edict {
    pub fn init(name: &str, conversion: Conversion) -> Edict {
        Edict {
            name: name.to_owned(),
            conversion,
            research: HashSet::new(),
            effective_range: 1,
            effective_bonus: 0,
        }
    }

    pub fn with_single_research(mut self, research: &str) -> Edict {
        self.research.insert(research.to_string());
        self
    }

    pub fn with_research(mut self, research: Vec<&str>) -> Edict {
        self.research = research.iter().map(|x| (*x).to_owned()).collect();
        self
    }

    pub fn with_effective_bonus(mut self, effective_bonus: u32) -> Edict {
        self.effective_bonus = effective_bonus;
        self
    }

    pub fn with_effective_range(mut self, effective_range: u32) -> Edict {
        self.effective_range = effective_range;
        self
    }

    pub fn is_available(&self, state: &GameState) -> bool {
        check_available_by_research(&self.research, &state)
    }
}
