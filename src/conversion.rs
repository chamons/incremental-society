use crate::resources::*;

const CONVERSION_TICK_START: u32 = 100;

#[derive(Debug)]
pub struct Conversion<'a> {
    pub name: &'a str,
    pub input: Vec<ResourceAmount>,
    pub output: Vec<ResourceAmount>,
    pub input_required_or_output: bool,
    pub ticks: u32,
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
            ticks: CONVERSION_TICK_START,
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
            ticks: CONVERSION_TICK_START,
        }
    }

    pub fn process_tick(&mut self, resources: &mut ResourceTotal) {
        self.convert(resources)
    }

    pub fn convert(&mut self, resources: &mut ResourceTotal) {
        if !self.is_ready() {
            self.ticks -= 1;
            return;
        }

        if self.input_required_or_output {
            self.convert_required(resources);
        } else {
            self.convert_optional(resources);
        }
        self.ticks = CONVERSION_TICK_START
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
        self.input.iter().all(|x| resources.has_amount(x))
    }

    pub fn is_ready(&self) -> bool {
        self.ticks <= 0
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

impl<'a> Clone for Conversion<'a> {
    fn clone(&self) -> Self {
        Conversion {
            name: self.name,
            input: self.input.clone(),
            output: self.output.clone(),
            input_required_or_output: self.input_required_or_output,
            ticks: self.ticks,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_conversion<'a>() -> Conversion<'a> {
        let mut c = Conversion::init_single(
            "TestConversion",
            ResourceAmount::init(ResourceKind::Food, 10),
            ResourceAmount::init(ResourceKind::Fuel, 10),
        );
        c.ticks = 0;
        c
    }

    fn create_test_required_conversion<'a>() -> Conversion<'a> {
        let mut c = Conversion::init_required_single(
            "TestRequiredConversion",
            ResourceAmount::init(ResourceKind::Food, 10),
            ResourceAmount::init(ResourceKind::Fuel, 10),
        );
        c.ticks = 0;
        c
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
    fn is_tick_ready() {
        let mut conversion = create_test_conversion();

        assert!(conversion.is_ready());
        conversion.ticks = CONVERSION_TICK_START;
        assert_eq!(false, conversion.is_ready());
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
        let mut conversion = create_test_conversion();

        conversion.convert(&mut resources);

        assert_eq!(0, resources[ResourceKind::Food]);
        assert_eq!(10, resources[ResourceKind::Fuel]);
        assert_eq!(CONVERSION_TICK_START, conversion.ticks);
    }

    #[test]
    fn conversion_without_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 5;
        let mut conversion = create_test_conversion();

        conversion.convert(&mut resources);

        assert_eq!(5, resources[ResourceKind::Food]);
        assert_eq!(0, resources[ResourceKind::Fuel]);
        assert_eq!(CONVERSION_TICK_START, conversion.ticks);
    }

    #[test]
    fn required_conversion_with_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 10;
        let mut conversion = create_test_required_conversion();

        conversion.convert(&mut resources);

        assert_eq!(0, resources[ResourceKind::Food]);
        assert_eq!(0, resources[ResourceKind::Fuel]);
        assert_eq!(CONVERSION_TICK_START, conversion.ticks);
    }

    #[test]
    fn required_conversion_without_input() {
        let mut resources = ResourceTotal::init();
        let mut conversion = create_test_required_conversion();

        conversion.convert(&mut resources);

        assert_eq!(0, resources[ResourceKind::Food]);
        assert_eq!(10, resources[ResourceKind::Fuel]);
        assert_eq!(CONVERSION_TICK_START, conversion.ticks);
    }

    #[test]
    fn conversion_without_ticks() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 10;
        let mut conversion = create_test_conversion();
        conversion.ticks = CONVERSION_TICK_START;

        conversion.convert(&mut resources);

        assert_eq!(10, resources[ResourceKind::Food]);
        assert_eq!(0, resources[ResourceKind::Fuel]);
        assert_eq!(CONVERSION_TICK_START - 1, conversion.ticks);
    }
}
