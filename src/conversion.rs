use crate::resources::*;

#[derive(Debug, Clone)]
pub enum ConversionLength {
    Short,
    Medium,
    Long,
    Epic,
}

#[derive(Debug, Clone)]
pub struct Conversion {
    pub name: &'static str,
    pub length: ConversionLength,
    pub input: Vec<ResourceAmount>,
    pub output: Vec<ResourceAmount>,
    pub output_if_no_input: Vec<ResourceAmount>,
}

impl Conversion {
    pub fn init_single(name: &'static str, length: ConversionLength, input: ResourceAmount, output: ResourceAmount) -> Conversion {
        Conversion::init(name, length, vec![input], vec![output])
    }

    pub fn init(name: &'static str, length: ConversionLength, input: Vec<ResourceAmount>, output: Vec<ResourceAmount>) -> Conversion {
        Conversion {
            name,
            length,
            input,
            output,
            output_if_no_input: vec![],
        }
    }

    pub fn init_required_single(
        name: &'static str,
        length: ConversionLength,
        input: ResourceAmount,
        output: ResourceAmount,
        output_if_no_input: ResourceAmount,
    ) -> Conversion {
        Conversion::init_required(name, length, vec![input], vec![output], vec![output_if_no_input])
    }

    pub fn init_required(
        name: &'static str,
        length: ConversionLength,
        input: Vec<ResourceAmount>,
        output: Vec<ResourceAmount>,
        output_if_no_input: Vec<ResourceAmount>,
    ) -> Conversion {
        Conversion {
            name,
            length,
            input,
            output,
            output_if_no_input,
        }
    }

    pub fn convert(&self, resources: &mut ResourceTotal) {
        if self.output_if_no_input.len() > 0 {
            self.convert_required(resources);
        } else {
            self.convert_optional(resources);
        }
    }

    // If we have input, consume it, else apply "bad" output
    fn convert_required(&self, resources: &mut ResourceTotal) {
        if self.has_input(&resources) {
            resources.remove_range(&self.input);
            resources.add_range(&self.output);
        } else {
            resources.add_range(&self.output_if_no_input);
        }
    }

    // If we have input, consume and apply "good" output
    fn convert_optional(&self, resources: &mut ResourceTotal) {
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
            ConversionLength::Short => 50,
            ConversionLength::Medium => 100,
            ConversionLength::Long => 200,
            ConversionLength::Epic => 400,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_conversion() -> Conversion {
        Conversion::init_single(
            "TestConversion",
            ConversionLength::Medium,
            ResourceAmount::init(ResourceKind::Food, 10),
            ResourceAmount::init(ResourceKind::Fuel, 10),
        )
    }

    fn create_test_required_conversion() -> Conversion {
        Conversion::init_required_single(
            "TestRequiredConversion",
            ConversionLength::Medium,
            ResourceAmount::init(ResourceKind::Food, 10),
            ResourceAmount::init(ResourceKind::Fuel, 10),
            ResourceAmount::init(ResourceKind::Knowledge, 1),
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
    fn required_conversion_with_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 10;
        let conversion = create_test_required_conversion();

        conversion.convert(&mut resources);

        assert_eq!(0, resources[ResourceKind::Food]);
        assert_eq!(10, resources[ResourceKind::Fuel]);
        assert_eq!(0, resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn required_conversion_without_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 5;
        let conversion = create_test_required_conversion();

        conversion.convert(&mut resources);

        assert_eq!(5, resources[ResourceKind::Food]);
        assert_eq!(0, resources[ResourceKind::Fuel]);
        assert_eq!(1, resources[ResourceKind::Knowledge]);
    }

    #[test]
    fn conversion_tick_length() {
        let conversion = create_test_conversion();
        assert!(conversion.tick_length() > 0)
    }
}
