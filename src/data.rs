use crate::state::{Building, Conversion, ConversionLength, Research, ResourceAmount, ResourceKind};

use std::collections::HashMap;

pub const BUILD_LENGTH: u32 = 30 * 8;
pub const SUSTAIN_POP_DURATION: u32 = 80;
pub const DESTROY_LENGTH: u32 = 30 * 5;
pub const REGION_BUILDING_COUNT: usize = 2;

pub const SHORT_CONVERSION: u32 = 50;
pub const MEDIUM_CONVERSION: u32 = 100;
pub const LONG_CONVERSION: u32 = 150;
pub const EPIC_CONVERSION: u32 = 300;

macro_rules! set(
    { $($key:expr),* } => {
        {
            #[allow(unused_mut)]
            let mut m = ::std::collections::HashSet::new();
            $(
                m.insert($key.to_owned());
            )*
            m
        }
     };
);

#[cfg(not(test))]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
        m.insert(
            "Sustain Population",
            Conversion::init_required(
                "Sustain Population",
                ConversionLength::Short,
                vec![ResourceAmount::init(ResourceKind::Food, 1)],
                vec![ResourceAmount::init(ResourceKind::Instability, -1)],
                vec![ResourceAmount::init(ResourceKind::Instability, 15)],
            ),
        );
        m.insert(
            "Gathering",
            Conversion::init(
                "Gathering",
                ConversionLength::Long,
                vec![],
                vec![ResourceAmount::init(ResourceKind::Food, 1), ResourceAmount::init(ResourceKind::Fuel, 1)],
            ),
        );

        m.insert(
            "Hunting",
            Conversion::init("Hunting", ConversionLength::Medium, vec![], vec![ResourceAmount::init(ResourceKind::Food, 2)]),
        );
        m.insert(
            "Feast",
            Conversion::init(
                "Feast",
                ConversionLength::Epic,
                vec![ResourceAmount::init(ResourceKind::Food, 20)],
                vec![ResourceAmount::init(ResourceKind::Knowledge, 5)],
            ),
        );

        m
    };
    static ref BUILDINGS: HashMap<&'static str, Building> = {
        let mut m: HashMap<&'static str, Building> = HashMap::new();
        m.insert(
            "Settlement",
            Building::init("Settlement")
                .with_conversions(vec!["Hunting"])
                .with_storage(vec![
                    ResourceAmount::init(ResourceKind::Food, 50),
                    ResourceAmount::init(ResourceKind::Fuel, 50),
                    ResourceAmount::init(ResourceKind::Knowledge, 50),
                    ResourceAmount::init(ResourceKind::Instability, 50),
                ])
                .with_pops(3)
                .as_immortal(),
        );

        m.insert(
            "Gathering Camp",
            Building::init("Gathering Camp")
                .with_conversions(vec!["Gathering", "Gathering", "Hunting"])
                .with_build_cost(vec![ResourceAmount::init(ResourceKind::Fuel, 0)])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Fuel, 25)])
                .with_pops(3),
        );

        m.insert(
            "Hunting Grounds",
            Building::init("Hunting Grounds")
                .with_conversions(vec!["Hunting"])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 20)]),
        );

        m
    };
    static ref EDICTS: Vec<&'static str> = {
        let mut e = Vec::new();
        e.push("Feast");
        e
    };
    static ref RESEARCH: HashMap<&'static str, Research> = {
        let mut m = HashMap::new();
        m.insert(
            "X",
            Research {
                name: "X".to_owned(),
                dependencies: set![],
            },
        );
        m
    };
}

#[cfg(test)]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
        m.insert(
            "Sustain Population",
            Conversion::init("Sustain Population", ConversionLength::Medium, vec![], vec![]),
        );
        m.insert(
            "TestChop",
            Conversion::init("TestChop", ConversionLength::Medium, vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 1)]),
        );
        m.insert(
            "TestGather",
            Conversion::init(
                "TestGather",
                ConversionLength::Medium,
                vec![],
                vec![ResourceAmount::init(ResourceKind::Food, 1)],
            ),
        );
        m.insert(
            "TestEdict",
            Conversion::init(
                "TestEdict",
                ConversionLength::Short,
                vec![ResourceAmount::init(ResourceKind::Fuel, 1)],
                vec![ResourceAmount::init(ResourceKind::Knowledge, 1)],
            ),
        );
        m.insert("OtherTestEdict", Conversion::init("OtherTestEdict", ConversionLength::Short, vec![], vec![]));
        m.insert(
            "TestHunt",
            Conversion::init("TestHunt", ConversionLength::Medium, vec![], vec![ResourceAmount::init(ResourceKind::Food, 2)]),
        );
        m
    };
    static ref BUILDINGS: HashMap<&'static str, Building> = {
        let mut m = HashMap::new();
        m.insert("Empty Building", Building::init("Empty Building"));
        m.insert(
            "Test Building",
            Building::init("Test Building")
                .with_conversions(vec!["TestChop", "TestChop"])
                .with_build_cost(vec![ResourceAmount::init(ResourceKind::Fuel, 10)])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Fuel, 15)])
                .with_pops(2),
        );
        m.insert(
            "Test Gather Hut",
            Building::init("Test Gather Hut")
                .with_conversions(vec!["TestGather"])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 20)]),
        );

        m.insert(
            "Test Hunt Cabin",
            Building::init("Test Hunt Cabin")
                .with_conversions(vec!["TestHunt", "TestHunt"])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 20)]),
        );

        m.insert(
            "Stability Building",
            Building::init("Stability Building").with_storage(vec![
                ResourceAmount::init(ResourceKind::Knowledge, 10),
                ResourceAmount::init(ResourceKind::Instability, 10),
            ]),
        );

        m.insert("Test Immortal", Building::init("Test Immortal").as_immortal());

        m
    };
    static ref EDICTS: Vec<&'static str> = {
        let mut e = Vec::new();
        e.push("TestEdict");
        e
    };
    static ref RESEARCH: HashMap<&'static str, Research> = {
        let mut m = HashMap::new();

        m.insert(
            "TestNoDeps",
            Research {
                name: "TestNoDeps".to_owned(),
                dependencies: set![],
            },
        );
        m.insert(
            "Dep",
            Research {
                name: "Dep".to_owned(),
                dependencies: set![],
            },
        );
        m.insert(
            "TestWithDep",
            Research {
                name: "TestWithDep".to_owned(),
                dependencies: set!["Dep"],
            },
        );
        m
    };
}

pub fn get_conversion(name: &str) -> Conversion {
    CONVERSIONS[name].clone()
}

pub fn get_building(name: &str) -> Building {
    BUILDINGS[name].clone()
}

pub fn get_building_names() -> Vec<String> {
    BUILDINGS
        .iter()
        .filter(|(_, building)| !building.immortal)
        .map(|(name, _)| (*name).to_string())
        .collect()
}

pub fn get_edict(name: &str) -> Conversion {
    CONVERSIONS[name].clone()
}

pub fn get_edict_names() -> Vec<String> {
    EDICTS.iter().map(|x| (*x).to_string()).collect()
}

pub fn get_research(name: &str) -> Research {
    RESEARCH[name].clone()
}

pub fn get_research_names() -> Vec<String> {
    RESEARCH.keys().map(|x| (*x).to_string()).collect()
}
