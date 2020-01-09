use crate::building::Building;
use crate::conversion::Conversion;

#[allow(unused_imports)] // Used in non-test version
use crate::resources::*;
use std::collections::HashMap;

#[cfg(not(test))]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
        m.insert(
            "Sustain Population",
            Conversion::init_required(
                "Sustain Population",
                vec![ResourceAmount::init(ResourceKind::Food, 1)],
                vec![ResourceAmount::init(ResourceKind::Instability, -1)],
                vec![ResourceAmount::init(ResourceKind::Instability, 15)],
            ),
        );
        m.insert(
            "Gathering",
            Conversion::init(
                "Gathering",
                vec![],
                vec![ResourceAmount::init(ResourceKind::Food, 1), ResourceAmount::init(ResourceKind::Fuel, 1)],
            ),
        );

        m.insert(
            "Hunting",
            Conversion::init("Hunting", vec![], vec![ResourceAmount::init(ResourceKind::Food, 2)]),
        );
        m.insert(
            "Feast",
            Conversion::init(
                "Feast",
                vec![ResourceAmount::init(ResourceKind::Food, 20)],
                vec![ResourceAmount::init(ResourceKind::Knowledge, 5)],
            ),
        );

        m
    };
    static ref BUILDINGS: HashMap<&'static str, Building> = {
        let mut m = HashMap::new();
        {
            let mut building = Building::init(
                "Settlement",
                vec!["Hunting"],
                vec![],
                vec![
                    ResourceAmount::init(ResourceKind::Food, 50),
                    ResourceAmount::init(ResourceKind::Fuel, 50),
                    ResourceAmount::init(ResourceKind::Knowledge, 50),
                    ResourceAmount::init(ResourceKind::Instability, 50),
                ],
                3,
            );
            building.immortal = true;
            m.insert("Settlement", building);
        }
        m.insert(
            "Gathering Camp",
            Building::init(
                "Gathering Camp",
                vec!["Gathering", "Gathering"],
                vec![],
                vec![ResourceAmount::init(ResourceKind::Fuel, 25)],
                3,
            ),
        );
        m.insert(
            "Hunting Grounds",
            Building::init(
                "Hunting Grounds",
                vec!["Hunting"],
                vec![],
                vec![ResourceAmount::init(ResourceKind::Food, 20)],
                0,
            ),
        );

        m
    };
    static ref EDICTS: Vec<&'static str> = {
        let mut e = Vec::new();
        e.push("Feast");
        e
    };
}

#[cfg(test)]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
        m.insert("Sustain Population", Conversion::init("Sustain Population", vec![], vec![]));
        m.insert(
            "TestChop",
            Conversion::init("TestChop", vec![], vec![ResourceAmount::init(ResourceKind::Fuel, 1)]),
        );
        m.insert(
            "TestGather",
            Conversion::init("TestGather", vec![], vec![ResourceAmount::init(ResourceKind::Food, 1)]),
        );
        m.insert(
            "TestEdict",
            Conversion::init(
                "TestEdict",
                vec![ResourceAmount::init(ResourceKind::Fuel, 1)],
                vec![ResourceAmount::init(ResourceKind::Knowledge, 1)],
            ),
        );
        m
    };
    static ref BUILDINGS: HashMap<&'static str, Building> = {
        let mut m = HashMap::new();
        m.insert("Empty Building", Building::init("Empty Building", vec![], vec![], vec![], 0));
        m.insert(
            "Test Building",
            Building::init(
                "Test Building",
                vec!["TestChop", "TestChop"],
                vec![ResourceAmount::init(ResourceKind::Fuel, 10)],
                vec![ResourceAmount::init(ResourceKind::Fuel, 15)],
                2,
            ),
        );
        m.insert(
            "Test Gather Hut",
            Building::init(
                "Test Gather Hut",
                vec!["TestGather"],
                vec![],
                vec![ResourceAmount::init(ResourceKind::Food, 20)],
                0,
            ),
        );

        m.insert(
            "Stability Building",
            Building::init(
                "Stability Building",
                vec![""],
                vec![],
                vec![
                    ResourceAmount::init(ResourceKind::Knowledge, 10),
                    ResourceAmount::init(ResourceKind::Instability, 10),
                ],
                0,
            ),
        );

        {
            let mut building = Building::init("Test Immortal", vec![""], vec![], vec![], 0);
            building.immortal = true;
            m.insert("Test Immortal", building);
        }
        m
    };
    static ref EDICTS: Vec<&'static str> = {
        let mut e = Vec::new();
        e.push("TestEdict");
        e
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
        .map(|(name, _)| name.to_string())
        .collect()
}

pub fn get_edict(name: &str) -> Conversion {
    CONVERSIONS[name].clone()
}

pub fn get_edict_names() -> Vec<String> {
    EDICTS.iter().map(|x| x.to_string()).collect()
}
