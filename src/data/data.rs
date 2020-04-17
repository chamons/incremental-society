use crate::state::{Building, Conversion, ConversionLength, Edict, Research, ResourceAmount, ResourceKind, Upgrade};

#[cfg(test)]
use crate::state::UpgradeActions;

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
                vec![ResourceAmount::init(ResourceKind::Food, 5), ResourceAmount::init(ResourceKind::Wood, 1)],
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
                Conversion::init("Hunt", ConversionLength::Long, vec![], vec![ResourceAmount::init(ResourceKind::Food, 20)]),
            ),
        );

        e
    };
    pub static ref RESEARCH: HashMap<&'static str, Research> = {
        let mut m = HashMap::new();
        m.insert(
            "Settlement",
            Research::init("Settlement").with_cost(vec![ResourceAmount::init(ResourceKind::Knowledge, 10)]),
        );
        m
    };
    pub static ref UPGRADE: HashMap<&'static str, Upgrade> = {
        let mut m = HashMap::new();
        m.insert("c", Upgrade::init("c", vec![], vec![]));
        m
    };
}
