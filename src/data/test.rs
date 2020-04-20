use crate::state::{Building, Conversion, ConversionLength, Edict, Research, ResourceAmount, ResourceKind, Upgrade, UpgradeActions};

use std::collections::HashMap;

#[cfg(test)]
lazy_static! {
    pub static ref AGES: Vec<&'static str> = { vec!["Archaic", "Stone", "Agricultural"] };
    pub static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
        m.insert(
            "TestChop",
            Conversion::init("TestChop", ConversionLength::Medium, vec![], vec![ResourceAmount::init(ResourceKind::Wood, 1)]),
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
                .with_jobs(vec!["TestChop", "TestChop"])
                .with_build_cost(vec![ResourceAmount::init(ResourceKind::Wood, 10)])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Wood, 15)])
                .with_housing(2),
        );
        m.insert(
            "Test Gather Hut",
            Building::init("Test Gather Hut")
                .with_jobs(vec!["TestGather"])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 120)]),
        );

        m.insert(
            "Test Hunt Cabin",
            Building::init("Test Hunt Cabin")
                .with_jobs(vec!["TestHunt", "TestHunt"])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 60)]),
        );

        m.insert(
            "Stability Building",
            Building::init("Stability Building").with_storage(vec![
                ResourceAmount::init(ResourceKind::Food, 300),
                ResourceAmount::init(ResourceKind::Knowledge, 30),
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
                    vec![ResourceAmount::init(ResourceKind::Wood, 1)],
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
            .with_single_research("TestNoDeps"),
        );
        e.insert(
            "TestEdictWithRange",
            Edict::init(
                "TestEdictWithRange",
                Conversion::init(
                    "TestEdictWithRange",
                    ConversionLength::Short,
                    vec![],
                    vec![ResourceAmount::init(ResourceKind::Knowledge, 10)],
                )
            ).with_effective_range(4) /* .25 - 4x*/
        );
        e.insert(
            "TestEdictWithRangeBonus",
            Edict::init(
                "TestEdictWithRangeBonus",
                Conversion::init(
                    "TestEdictWithRangeBonus",
                    ConversionLength::Short,
                    vec![],
                    vec![ResourceAmount::init(ResourceKind::Knowledge, 10)],
                )
            ).with_effective_range(4).with_effective_bonus(1.0) /* 1.25 - 5x*/
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
        m.insert("TestWithDep", Research::init("TestWithDep").with_dependency("Dep"));
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
            Upgrade::init_single(
                "TestUpgrade",
                "Test Building",
                vec![UpgradeActions::AddBuildingJob("TestChop".to_owned())]
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert(
            "TestEdictUpgrade",
            Upgrade::init_single(
                "TestEdictUpgrade",
                "TestEdict",
                vec![UpgradeActions::ChangeEdictLength(ConversionLength::Long)],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert(
            "TestEdictUpgradeYield",
            Upgrade::init_single(
                "TestEdictUpgradeYield",
                "TestEdict",
                vec![UpgradeActions::AddEdictBonus(1.0)],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert(
            "TestConversionUpgrade",
            Upgrade::init_single(
                "TestConversionUpgrade",
                "TestChop",
                vec![UpgradeActions::ChangeConversionOutput(ResourceAmount::init(ResourceKind::Knowledge, 1))],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert(
            "TestMultiUpgrade",
            Upgrade::init(
                "TestMultiUpgrade",
                vec!["Test Building".to_owned(), "TestEdict".to_owned(), "TestChop".to_owned()],
                vec![
                    UpgradeActions::AddBuildingJob("TestChop".to_owned()),
                    UpgradeActions::ChangeEdictLength(ConversionLength::Long),
                    UpgradeActions::ChangeConversionOutput(ResourceAmount::init(ResourceKind::Knowledge, 1)),
                ],
            )
            .with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 25)]),
        );

        m.insert("TestOtherUpgrade", Upgrade::init("TestOtherUpgrade", vec![], vec![]));

        m.insert(
            "TestUpgradeWithDep",
            Upgrade::init("TestUpgradeWithDep", vec![], vec![]).with_single_research("UpgradeTech"),
        );

        m.insert(
            "StabilityUpgrade",
            Upgrade::init(
                "StabilityUpgrade",
                vec![],
                vec![
                    UpgradeActions::ImproveStabilityGain(2),
                ]
            )
        );

        m.insert(
            "OtherStabilityUpgrade",
            Upgrade::init(
                "OtherStabilityUpgrade",
                vec![],
                vec![
                    UpgradeActions::ImproveStabilityGain(1),
                ]
            )
        );

        m
    };
}
