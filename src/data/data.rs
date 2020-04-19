use crate::state::{Building, Conversion, ConversionLength, Edict, Research, ResourceAmount, ResourceKind, Upgrade, UpgradeActions};
use std::collections::HashMap;

#[cfg(not(test))]
lazy_static! {
    pub static ref AGES: Vec<&'static str> = { vec!["Archaic", "Stone", "Agricultural"] };
    pub static ref CONVERSIONS: HashMap<&'static str, Conversion> = {
        let mut m = HashMap::new();
        m.insert(
            "Gathering",
            Conversion::init(
                "Gathering",
                ConversionLength::Long,
                vec![],
                vec![
                    ResourceAmount::init(ResourceKind::Food, 5),
                    ResourceAmount::init(ResourceKind::Wood, 1),
                    ResourceAmount::init(ResourceKind::Stone, 1),
                ],
            ),
        );

        m.insert(
            "Hunting",
            Conversion::init("Hunting", ConversionLength::Medium, vec![], vec![ResourceAmount::init(ResourceKind::Food, 10)]),
        );

        m
    };
    pub static ref BUILDINGS: HashMap<&'static str, Building> = {
        let mut m: HashMap<&'static str, Building> = HashMap::new();
        m.insert(
            "Settlement",
            Building::init("Settlement")
                .with_storage(vec![
                    ResourceAmount::init(ResourceKind::Food, 250),
                    ResourceAmount::init(ResourceKind::Wood, 50),
                    ResourceAmount::init(ResourceKind::Knowledge, 50),
                    ResourceAmount::init(ResourceKind::Instability, 50),
                ])
                .with_housing(3)
                .with_immortal(),
        );

        /*
        m.insert(
            "Gathering Camp",
            Building::init("Gathering Camp")
                .with_jobs(vec!["Gathering", "Gathering", "Hunting"])
                .with_build_cost(vec![ResourceAmount::init(ResourceKind::Wood, 0)])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Wood, 25)])
                .with_research("Settlement"),
        );

        m.insert(
            "Hunting Grounds",
            Building::init("Hunting Grounds")
                .with_jobs(vec!["Hunting"])
                .with_storage(vec![ResourceAmount::init(ResourceKind::Food, 20)])
                .with_research("Settlement"),
        );
        */

        m
    };
    pub static ref EDICTS: HashMap<&'static str, Edict> = {
        let mut e: HashMap<&'static str, Edict> = HashMap::new();
        e.insert(
            "Feast",
            Edict::init(
                "Feast",
                Conversion::init(
                    "Feast",
                    ConversionLength::Long,
                    vec![ResourceAmount::init(ResourceKind::Food, 50)],
                    vec![ResourceAmount::init(ResourceKind::Knowledge, 5)],
                ),
            ),
        );
        e.insert(
            "Hunt",
            Edict::init(
                "Hunt",
                Conversion::init("Hunt", ConversionLength::Long, vec![], vec![ResourceAmount::init(ResourceKind::Food, 20)])
            ).with_effective_range(3),
        );
        e.insert(
            "Gathering",
            Edict::init(
                "Gathering",
                Conversion::init("Gathering", ConversionLength::Medium, vec![], vec![ResourceAmount::init(ResourceKind::Food, 10),
                ResourceAmount::init(ResourceKind::Wood, 1),
                ResourceAmount::init(ResourceKind::Stone, 1)
            ])).with_effective_range(2).with_single_research("Gathering")
        );
        e
    };
    pub static ref RESEARCH: HashMap<&'static str, Research> = {
        let mut m = HashMap::new();
        m.insert(
            "Gathering",
            Research::init("Gathering").with_description("Expand the tribe's expeditions to collect plants and wild grains. Also obtain stone and wood where readily available.").with_knowledge_cost(10)
        );
        m.insert(
            "Tool Making",
            Research::init("Tool Making").with_dependencies(vec!["Gathering"]).with_description("Since times immemorial what separates mankind from the animal kingdom is the consistent use of tools. Unlocks fashioning tools from bone, stone, and wood.").with_knowledge_cost(20)
        );
        m.insert(
            "Seasonal Gathering",
            Research::init("Seasonal Gathering").with_dependencies(vec!["Gathering"]).with_description("By migrating along consistent routes, resources can be exploited in season and yield increased.").with_knowledge_cost(20)
        );
        m.insert(
            "Stone Spears",
            Research::init("Stone Spears").with_dependencies(vec!["Tool Making"]),
        );
        m.insert(
            "Spear Throwers",
            Research::init("Spear Throwers").with_dependencies(vec!["Stone Spears"]),
        );
        m.insert(
            "Stone Grinders",
            Research::init("Stone Grinders").with_dependencies(vec!["Tool Making"]),
        );
        m.insert(
            "Early Settlements",
            Research::init("Early Settlements").with_dependencies(vec!["Gathering"]),
        );
        m.insert(
            "Domestication of Dogs",
            Research::init("Domestication of Dogs").with_dependencies(vec!["Gathering"]),
        );
        m.insert(
            "Pigments",
            Research::init("Pigments").with_dependencies(vec!["Gathering"]),
        );

        m
    };
    pub static ref UPGRADE: HashMap<&'static str, Upgrade> = {
        let mut m = HashMap::new();
        m.insert("Seasonal Gathering", Upgrade::init("Seasonal Gathering", vec![ UpgradeActions::ChangeEdictLength(ConversionLength::Long), UpgradeActions::AddEdictBonus(2) ], vec!["Gathering".to_string()]));
        m
    };
}
