use std::ops::{Index, IndexMut};

type ResourceQuantity = i64;

#[derive(Debug, Copy, Clone)]
pub enum ResourceKind {
    Food,
    Fuel,

    // This must be incremented every time an item is added
    Size = 2,
}

#[derive(Debug, Copy, Clone)]
pub struct ResourceAmount {
    pub kind: ResourceKind,
    pub amount: ResourceQuantity,
}

impl ResourceAmount {
    pub fn init(kind: ResourceKind, amount: ResourceQuantity) -> ResourceAmount {
        ResourceAmount { kind, amount }
    }
}

#[derive(Debug)]
pub struct ResourceTotal {
    pub resources: [ResourceQuantity; NUM_RESOURCES],
}

impl ResourceTotal {
    pub fn init() -> ResourceTotal {
        ResourceTotal {
            resources: [0; NUM_RESOURCES],
        }
    }

    pub fn has(&self, resource: ResourceKind, amount: ResourceQuantity) -> bool {
        self[resource] >= amount
    }

    pub fn add(&mut self, resource: ResourceKind, amount: ResourceQuantity) {
        self[resource] += amount;
    }

    pub fn remove(&mut self, resource: ResourceKind, amount: ResourceQuantity) {
        debug_assert!(self.has(resource, amount));
        self[resource] -= amount;
    }

    pub fn combine(&mut self, other: &ResourceTotal) {
        for i in 0..NUM_RESOURCES {
            self[i] += other[i];
        }
    }
}

impl Index<ResourceKind> for ResourceTotal {
    type Output = ResourceQuantity;

    fn index(&self, resource: ResourceKind) -> &ResourceQuantity {
        &self.resources[resource as usize]
    }
}

impl Index<usize> for ResourceTotal {
    type Output = ResourceQuantity;

    fn index(&self, index: usize) -> &ResourceQuantity {
        &self.resources[index]
    }
}

impl IndexMut<ResourceKind> for ResourceTotal {
    fn index_mut(&mut self, resource: ResourceKind) -> &mut ResourceQuantity {
        &mut self.resources[resource as usize]
    }
}

impl IndexMut<usize> for ResourceTotal {
    fn index_mut(&mut self, index: usize) -> &mut ResourceQuantity {
        &mut self.resources[index]
    }
}

const NUM_RESOURCES: usize = ResourceKind::Size as usize;

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
    pub fn has_input(&self, state: &GameState) -> bool {
        self.input
            .iter()
            .all(|x| state.resources.has(x.kind, x.amount))
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

#[derive(Debug)]
pub struct GameState<'a> {
    pub resources: ResourceTotal,
    pub conversions: Vec<Conversion<'a>>,
}

impl<'a> GameState<'a> {
    pub fn init() -> GameState<'a> {
        GameState {
            resources: ResourceTotal::init(),
            conversions: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_total_has_enough() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Fuel] = 5;
        assert_eq!(true, total.has(ResourceKind::Fuel, 5));
        assert_eq!(false, total.has(ResourceKind::Fuel, 15));
        assert_eq!(false, total.has(ResourceKind::Food, 1));
    }

    #[test]
    fn resource_total_add() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Fuel] = 5;
        assert_eq!(true, total.has(ResourceKind::Fuel, 5));
        total.add(ResourceKind::Fuel, 10);
        assert_eq!(true, total.has(ResourceKind::Fuel, 15));
    }

    #[test]
    fn resource_total_remove() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Fuel] = 5;
        assert_eq!(true, total.has(ResourceKind::Fuel, 5));
        total.remove(ResourceKind::Fuel, 4);
        assert_eq!(true, total.has(ResourceKind::Fuel, 1));
    }
}
