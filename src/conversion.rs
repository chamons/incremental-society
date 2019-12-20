use crate::resources::*;

#[derive(Debug)]
pub struct Conversion<'a> {
    pub name: &'a str,
    pub input: Vec<ResourceAmount>,
    pub output: Vec<ResourceAmount>,
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
            name: name,
            input: input,
            output: output,
        }
    }
}

impl<'a> Conversion<'a> {
    pub fn has_input(&self, resources: &ResourceTotal) -> bool {
        self.input.iter().all(|x| resources.has(x.kind, x.amount))
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
    #[test]
    fn has_input() {
        let mut resources = ResourceTotal::init();
        resources[ResourceKind::Food] = 10;
        resources[ResourceKind::Fuel] = 5;

        let conversion = Conversion::init_single(
            "TestConversion",
            ResourceAmount::init(ResourceKind::Food, 5),
            ResourceAmount::init(ResourceKind::Fuel, 10),
        );

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
    }
}
