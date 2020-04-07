use crate::state::{Building, Conversion, ConversionLength, Edict, Research, ResourceAmount, ResourceKind, Upgrade, UpgradeActions};

use std::collections::HashMap;

#[cfg(test)]
lazy_static! {
    pub static ref AGES: Vec<&'static str> = { vec!["Archaic", "Stone", "Agricultural"] };
    pub static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
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
    pub static ref BUILDINGS: HashMap<&'static str, Building> = {
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
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 120)]),
        );

        m.insert(
            "Test Hunt Cabin",
            Building::init("Test Hunt Cabin")
                .with_conversions(vec!["TestHunt", "TestHunt"])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 60)]),
        );

        m.insert(
            "Stability Building",
            Building::init("Stability Building").with_storage(vec![
                ResourceAmount::init(ResourceKind::Food, 30),
                ResourceAmount::init(ResourceKind::Knowledge, 10),
                ResourceAmount::init(ResourceKind::Instability, 10),
            ]),
        );

        m.insert(
            "Requires Research Building",
            Building::init("Requires Research Building").with_research("TestNoDeps"),
        );

        m.insert("Test Immortal", Building::init("Test Immortal").with_immortal());

        m
    };
    pub static ref EDICTS: HashMap<&'static str, Edict> = {
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
    pub static ref RESEARCH: HashMap<&'static str, Research> = {
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
    pub static ref UPGRADE: HashMap<&'static str, Upgrade> = {
        let mut m = HashMap::new();

        m.insert(
            "TestUpgrade",
            Upgrade::init(
                "TestUpgrade",
                vec![UpgradeActions::AddBuildingConversion("TestChop".to_owned())],
                vec!["Test Building".to_owned()],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert(
            "TestEdictUpgrade",
            Upgrade::init(
                "TestEdictUpgrade",
                vec![UpgradeActions::ChangeEdictLength(ConversionLength::Long)],
                vec!["TestEdict".to_owned()],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert(
            "TestConversionUpgrade",
            Upgrade::init(
                "TestConversionUpgrade",
                vec![UpgradeActions::ChangeConversionOutput(ResourceAmount::init(ResourceKind::Knowledge, 1))],
                vec!["TestChop".to_owned()],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert(
            "TestMultiUpgrade",
            Upgrade::init(
                "TestMultiUpgrade",
                vec![
                    UpgradeActions::AddBuildingConversion("TestChop".to_owned()),
                    UpgradeActions::ChangeEdictLength(ConversionLength::Long),
                    UpgradeActions::ChangeConversionOutput(ResourceAmount::init(ResourceKind::Knowledge, 1)),
                ],
                vec!["Test Building".to_owned(), "TestEdict".to_owned(), "TestChop".to_owned()],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert("TestOtherUpgrade", Upgrade::init("TestOtherUpgrade", vec![], vec![]));

        m.insert(
            "TestUpgradeWithDep",
            Upgrade::init("TestUpgradeWithDep", vec![], vec![]).with_research(vec!["UpgradeTech"]),
        );

        m
    };
}
