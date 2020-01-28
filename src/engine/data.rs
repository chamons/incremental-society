use crate::state::{Building, Conversion, ConversionLength, Edict, Research, ResourceAmount, ResourceKind, Upgrade, UpgradeActions};

use std::collections::HashMap;

#[cfg(not(test))]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
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
                .with_immortal(),
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
    static ref EDICTS: HashMap<&'static str, Edict> = {
        let mut e: HashMap<&'static str, Edict> = HashMap::new();
        e.insert(
            "Feast",
            Edict::init(
                "Feast",
                Conversion::init(
                    "Feast",
                    ConversionLength::Epic,
                    vec![ResourceAmount::init(ResourceKind::Food, 20)],
                    vec![ResourceAmount::init(ResourceKind::Knowledge, 5)],
                ),
            ),
        );

        e
    };
    static ref RESEARCH: HashMap<&'static str, Research> = {
        let mut m = HashMap::new();
        m.insert(
            "Settlement",
            Research::init("Settlement").with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 10)]),
        );
        m
    };
    static ref UPGRADE: HashMap<&'static str, Upgrade> = {
        let mut m = HashMap::new();
        m
    };
}

#[cfg(test)]
lazy_static! {
    static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
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

        m.insert(
            "Requires Research Building",
            Building::init("Requires Research Building").with_research(vec!["TestNoDeps"]),
        );

        m.insert("Test Immortal", Building::init("Test Immortal").with_immortal());

        m
    };
    static ref EDICTS: HashMap<&'static str, Edict> = {
        let mut e: HashMap<&'static str, Edict> = HashMap::new();
        e.insert(
            "TestEdict",
            Edict::init(
                "TestEdict",
                Conversion::init(
                    "TestEdict",
                    ConversionLength::Short,
                    vec![ResourceAmount::init(ResourceKind::Fuel, 1)],
                    vec![ResourceAmount::init(ResourceKind::Knowledge, 1)],
                ),
            ),
        );
        e.insert(
            "TestEdictWithResearch",
            Edict::init(
                "TestEdictWithResearch",
                Conversion::init("TestEdictWithResearch", ConversionLength::Short, vec![], vec![]),
            )
            .with_research(vec!["TestNoDeps"]),
        );
        e.insert(
            "OtherTestEdict",
            Edict::init("OtherTestEdict", Conversion::init("OtherTestEdict", ConversionLength::Short, vec![], vec![])),
        );

        e
    };
    static ref RESEARCH: HashMap<&'static str, Research> = {
        let mut m = HashMap::new();

        m.insert("TestNoDeps", Research::init("TestNoDeps"));
        m.insert("Dep", Research::init("Dep"));
        m.insert("TestWithDep", Research::init("TestWithDep").with_dependencies(vec!["Dep"]));
        m.insert(
            "TestWithCost",
            Research::init("TestWithCost").with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 10)]),
        );
        m.insert("UpgradeTech", Research::init("UpgradeTech"));

        m
    };
    static ref UPGRADE: HashMap<&'static str, Upgrade> = {
        let mut m = HashMap::new();

        m.insert(
            "TestUpgrade",
            Upgrade::init(
                "TestUpgrade",
                vec![UpgradeActions::AddBuildingConversion("TestChop".to_owned())],
                vec!["Test Building".to_owned()],
            ),
        );

        m.insert(
            "TestEdictUpgrade",
            Upgrade::init(
                "TestEdictUpgrade",
                vec![UpgradeActions::ChangeEdictLength(ConversionLength::Long)],
                vec!["TestEdict".to_owned()],
            ),
        );

        m.insert("TestOtherUpgrade", Upgrade::init("TestOtherUpgrade", vec![], vec![]));

        m.insert(
            "TestUpgradeWithDep",
            Upgrade::init("TestUpgradeWithDep", vec![], vec![]).with_research(vec!["UpgradeTech"]),
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

pub fn get_edict(name: &str) -> Edict {
    EDICTS[name].clone()
}

pub fn get_edict_names() -> Vec<String> {
    EDICTS.keys().map(|x| (*x).to_string()).collect()
}

pub fn get_research(name: &str) -> Research {
    RESEARCH[name].clone()
}

pub fn get_research_names() -> Vec<String> {
    RESEARCH.keys().map(|x| (*x).to_string()).collect()
}

pub fn get_upgrade(name: &str) -> Upgrade {
    UPGRADE[name].clone()
}

pub fn get_upgrade_names() -> Vec<String> {
    UPGRADE.keys().map(|x| (*x).to_string()).collect()
}
