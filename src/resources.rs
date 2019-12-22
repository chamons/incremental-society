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

pub const NUM_RESOURCES: usize = ResourceKind::Size as usize;

#[derive(Debug, Clone)]
pub struct ResourceTotal {
    pub resources: [ResourceQuantity; NUM_RESOURCES],
}

impl ResourceTotal {
    pub fn init() -> ResourceTotal {
        ResourceTotal { resources: [0; NUM_RESOURCES] }
    }

    pub fn has_amount(&self, amount: &ResourceAmount) -> bool {
        self.has(amount.kind, amount.amount)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_total_has_enough() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Fuel] = 5;
        assert!(total.has(ResourceKind::Fuel, 1));
        assert!(total.has(ResourceKind::Fuel, 5));
        assert_eq!(false, total.has(ResourceKind::Fuel, 15));
        assert_eq!(false, total.has(ResourceKind::Food, 1));
    }

    #[test]
    fn resource_total_add() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Fuel] = 5;
        assert!(total.has(ResourceKind::Fuel, 5));
        total.add(ResourceKind::Fuel, 10);
        assert!(total.has(ResourceKind::Fuel, 15));
    }

    #[test]
    fn resource_total_remove() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Fuel] = 5;
        assert!(total.has(ResourceKind::Fuel, 5));
        total.remove(ResourceKind::Fuel, 4);
        assert!(total.has(ResourceKind::Fuel, 1));
    }

    #[test]
    fn resource_combine() {
        let mut a = ResourceTotal::init();
        a[ResourceKind::Food] = 5;
        a[ResourceKind::Fuel] = 5;

        let mut b = ResourceTotal::init();
        b[ResourceKind::Food] = 5;
        a.combine(&b);

        assert_eq!(10, a[ResourceKind::Food]);
        assert_eq!(5, a[ResourceKind::Fuel]);
    }
}
