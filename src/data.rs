use crate::building::Building;
use crate::conversion::Conversion;

#[allow(unused_imports)] // Used in non-test version
use crate::resources::*;
use std::collections::HashMap;

#[cfg(not(test))]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion<'static>> = {
        let mut m = HashMap::new();
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

        m
    };
    static ref BUILDINGS: HashMap<&'static str, Building<'static>> = {
        let mut m = HashMap::new();
        m.insert("Gathering Camp", Building::init("Gathering Camp", vec!["Gathering", "Gathering"], vec![]));
        m.insert("Hunting Grounds", Building::init("Hunting Grounds", vec!["Hunting"], vec![]));

        m
    };
}

#[cfg(test)]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion<'static>> = {
        let mut m = HashMap::new();
        m.insert("TestEmptyConvert", Conversion::init("TestEmptyConvert", vec![], vec![]));
        m
    };
    static ref BUILDINGS: HashMap<&'static str, Building<'static>> = {
        let mut m = HashMap::new();
        m.insert(
            "Test Building",
            Building::init(
                "Test Building",
                vec!["TestEmptyConvert", "TestEmptyConvert"],
                vec![ResourceAmount::init(ResourceKind::Fuel, 10)],
            ),
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

pub fn get_building_names() -> Vec<&'static str> {
    BUILDINGS.keys().cloned().collect()
}
