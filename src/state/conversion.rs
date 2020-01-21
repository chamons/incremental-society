use std::fmt;

use super::resources::*;
use crate::data;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ConversionLength {
    Short,
    Medium,
    Long,
    Epic,
}

impl fmt::Display for ConversionLength {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Conversion {
    pub name: String,
    pub length: ConversionLength,
    pub input: Vec<ResourceAmount>,
    pub output: Vec<ResourceAmount>,
}

impl Conversion {
    pub fn init(name: &'static str, length: ConversionLength, input: Vec<ResourceAmount>, output: Vec<ResourceAmount>) -> Conversion {
        Conversion {
            name: name.to_owned(),
            length,
            input,
            output,
        }
    }

    pub fn convert(&self, resources: &mut ResourceTotal) {
        if self.has_input(&resources) {
            resources.remove_range(&self.input);
            resources.add_range(&self.output);
        }
    }

    pub fn has_input(&self, resources: &ResourceTotal) -> bool {
        self.input.iter().all(|x| resources.has_amount(x))
    }

    pub fn tick_length(&self) -> u32 {
        match &self.length {
            ConversionLength::Short => data::SHORT_CONVERSION,
            ConversionLength::Medium => data::MEDIUM_CONVERSION,
            ConversionLength::Long => data::LONG_CONVERSION,
            ConversionLength::Epic => data::EPIC_CONVERSION,
        }
    }

    pub fn details(&self) -> Vec<String> {
        let mut details: Vec<String> = vec![];
        details.push(format!(
            "Requires: {}",
            self.input.iter().map(|x| format!("{} {}", x.amount, x.kind)).format(", ")
        ));
        details.push(format!(
            "Provides: {}",
            self.output.iter().map(|x| format!("{} {}", x.amount, x.kind)).format(", ")
        ));
        details.push(format!("Length: {} ({})", self.length, self.tick_length()));
        details
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_conversion() -> Conversion {
        Conversion::init(
            "TestConversion",
            ConversionLength::Medium,
            vec![ResourceAmount::init(ResourceKind::Food, 10)],
            vec![ResourceAmount::init(ResourceKind::Fuel, 10)],
        )
    }

    #[test]
    fn has_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 10;
        resources[ResourceKind::Fuel] = 5;

        let conversion = create_test_conversion();

        assert!(conversion.has_input(&resources));
        resources[ResourceKind::Food] = 0;
        assert_eq!(false, conversion.has_input(&resources));
    }

    #[test]
    fn conversion_with_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 10;
        let conversion = create_test_conversion();

        conversion.convert(&mut resources);

        assert_eq!(0, resources[ResourceKind::Food]);
        assert_eq!(10, resources[ResourceKind::Fuel]);
    }

    #[test]
    fn conversion_without_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 5;
        let conversion = create_test_conversion();

        conversion.convert(&mut resources);

        assert_eq!(5, resources[ResourceKind::Food]);
        assert_eq!(0, resources[ResourceKind::Fuel]);
    }

    #[test]
    fn conversion_tick_length() {
        let conversion = create_test_conversion();
        assert!(conversion.tick_length() > 0)
    }
}
