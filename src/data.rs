use crate::building::*;
use crate::conversion::Conversion;
use crate::resources::*;

use std::collections::HashMap;

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
        m.insert(
            "Gathering Camp",
            Building::init(
                "Gathering Camp",
                vec![CONVERSIONS["Gathering"].clone(), CONVERSIONS["Gathering"].clone()],
                vec![],
            ),
        );
        m.insert(
            "Hunting Grounds",
            Building::init("Hunting Grounds", vec![CONVERSIONS["Hunting"].clone()], vec![]),
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
