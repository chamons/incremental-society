use crate::resources::*;

#[derive(Debug)]
pub struct Conversion<'a> {
    pub name: &'a str,
    pub input: Vec<ResourceAmount>,
    pub output: Vec<ResourceAmount>,
    pub input_required_or_output: bool,
}

impl<'a> Conversion<'a> {
    pub fn init_single(
        name: &'a str,
        input: ResourceAmount,
        output: ResourceAmount,
    ) -> Conversion<'a> {
        Conversion::init(name, vec![input], vec![output])
    }

    pub fn init(
        name: &'a str,
        input: Vec<ResourceAmount>,
        output: Vec<ResourceAmount>,
    ) -> Conversion<'a> {
        Conversion {
            name,
            input,
            output,
            input_required_or_output: false,
        }
    }

    pub fn init_required_single(
        name: &'a str,
        input: ResourceAmount,
        output: ResourceAmount,
    ) -> Conversion<'a> {
        Conversion::init_required(name, vec![input], vec![output])
    }

    pub fn init_required(
        name: &'a str,
        input: Vec<ResourceAmount>,
        output: Vec<ResourceAmount>,
    ) -> Conversion<'a> {
        Conversion {
            name,
            input,
            output,
            input_required_or_output: true,
        }
    }

    pub fn convert(&self, resources: &mut ResourceTotal) {
        if self.input_required_or_output {
            self.convert_required(resources);
        } else {
            self.convert_optional(resources);
        }
    }

    // If we have input, consume it, else apply "bad" output
    fn convert_required(&self, resources: &mut ResourceTotal) {
        if self.has_input(&resources) {
            resources.combine(&self.total_input())
        } else {
            resources.combine(&self.total_output())
        }
    }

    // If we have input, consume and apply "good" output
    fn convert_optional(&self, resources: &mut ResourceTotal) {
        if self.has_input(&resources) {
            resources.combine(&self.total());
        }
    }

    pub fn has_input(&self, resources: &ResourceTotal) -> bool {
        self.input.iter().all(|x| resources.has(x.kind, x.amount))
    }

    pub fn total_input(&self) -> ResourceTotal {
        let mut total = ResourceTotal::init();
        for i in self.input.iter() {
            total[i.kind] -= i.amount;
        }
        total
    }

    pub fn total_output(&self) -> ResourceTotal {
        let mut total = ResourceTotal::init();
        for i in self.output.iter() {
            total[i.kind] += i.amount;
        }
        total
    }

    pub fn total(&self) -> ResourceTotal {
        let mut total = ResourceTotal::init();
        for i in self.input.iter() {
            total[i.kind] -= i.amount;
        }
        for i in self.output.iter() {
            total[i.kind] += i.amount;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_conversion<'a>() -> Conversion<'a> {
        Conversion::init_single(
            "TestConversion",
            ResourceAmount::init(ResourceKind::Food, 10),
            ResourceAmount::init(ResourceKind::Fuel, 10),
        )
    }

    fn create_test_required_conversion<'a>() -> Conversion<'a> {
        Conversion::init_required_single(
            "TestRequiredConversion",
            ResourceAmount::init(ResourceKind::Food, 10),
            ResourceAmount::init(ResourceKind::Fuel, 10),
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
    fn total() {
        let conversion = Conversion::init_single(
            "TestConversion",
            ResourceAmount::init(ResourceKind::Food, 5),
            ResourceAmount::init(ResourceKind::Fuel, 10),
        );

        let total = conversion.total();
        assert_eq!(-5, total[ResourceKind::Food]);
        assert_eq!(10, total[ResourceKind::Fuel]);

        let total_input = conversion.total_input();
        assert_eq!(-5, total_input[ResourceKind::Food]);
        assert_eq!(0, total_input[ResourceKind::Fuel]);

        let total_output = conversion.total_output();
        assert_eq!(0, total_output[ResourceKind::Food]);
        assert_eq!(10, total_output[ResourceKind::Fuel]);
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
        assert_eq!(0, resources[ResourceKind::Fuel]);
    }

    #[test]
    fn required_conversion_without_input() {
        let mut resources = ResourceTotal::init();
        let conversion = create_test_required_conversion();

        conversion.convert(&mut resources);

        assert_eq!(0, resources[ResourceKind::Food]);
        assert_eq!(10, resources[ResourceKind::Fuel]);
    }
}
