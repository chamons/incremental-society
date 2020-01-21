use super::conversion::Conversion;

#[derive(Debug, Clone)]
pub struct Edict {
    pub name: String,
    pub conversion: Conversion,
    pub research: Vec<String>,
}

impl Edict {
    pub fn init(name: &str, conversion: Conversion) -> Edict {
        Edict {
            name: name.to_owned(),
            conversion,
            research: vec![],
        }
    }

    pub fn with_research(mut self, research: Vec<&str>) -> Edict {
        self.research = research.iter().map(|x| (*x).to_owned()).collect();
        self
    }
}
