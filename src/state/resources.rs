use std::fmt;
use std::ops::{Index, IndexMut};

use itertools::Itertools;
use num_traits::FromPrimitive;
use serde::{Deserialize, Serialize};

pub type ResourceQuantity = i64;

#[derive(Debug, Copy, Clone, FromPrimitive, ToPrimitive, Deserialize, Serialize, PartialEq)]
pub enum ResourceKind {
    Food,
    Wood,
    Stone,
    Knowledge,
    Instability,

    // This must be incremented every time an item is added
    Size = 5,
}

impl fmt::Display for ResourceKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResourceKind {
    pub fn name_for_index(index: usize) -> ResourceKind {
        ResourceKind::from_usize(index).unwrap()
    }
}

#[derive(Debug, Copy, Clone, Deserialize, Serialize)]
pub struct ResourceAmount {
    pub kind: ResourceKind,
    pub amount: ResourceQuantity,
}

impl fmt::Display for ResourceAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.amount, self.kind)
    }
}

impl ResourceAmount {
    pub fn init(kind: ResourceKind, amount: ResourceQuantity) -> ResourceAmount {
        ResourceAmount { kind, amount }
    }
}

pub const NUM_RESOURCES: usize = ResourceKind::Size as usize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceTotal {
    pub resources: [ResourceQuantity; NUM_RESOURCES],
}

impl ResourceTotal {
    pub fn init() -> ResourceTotal {
        ResourceTotal { resources: [0; NUM_RESOURCES] }
    }

    pub fn has_range(&self, elements: &[ResourceAmount]) -> bool {
        for x in elements {
            if self[x.kind] < x.amount {
                return false;
            }
        }
        true
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

    pub fn add_range(&mut self, elements: &[ResourceAmount]) {
        for x in elements {
            self.add(x.kind, x.amount);
        }
    }

    pub fn add_range_with_coefficient(&mut self, elements: &[ResourceAmount], m: f32) {
        for x in elements {
            self.add(x.kind, (m * x.amount as f32).ceil() as ResourceQuantity);
        }
    }

    pub fn remove(&mut self, resource: ResourceKind, amount: ResourceQuantity) {
        crate::engine::die_unless(
            self.has(resource, amount),
            &format!("Tried to remove {} {} from state which only had {}.", amount, resource, self[resource]),
        );
        self[resource] -= amount;
    }

    pub fn remove_range(&mut self, elements: &[ResourceAmount]) {
        for x in elements {
            self.remove(x.kind, x.amount);
        }
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

pub fn format_resource_list(prefix: &str, list: &[ResourceAmount]) -> String {
    let output_list = list.iter().map(|x| format!("{} {}", x.amount, x.kind)).format(", ");
    format!("{}{}", prefix, output_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_total_has_range() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Wood] = 5;

        let mut other = vec![ResourceAmount::init(ResourceKind::Wood, 10)];

        assert!(!total.has_range(&other));
        other[0].amount = 5;
        assert!(total.has_range(&other));
    }

    #[test]
    fn resource_total_has_enough() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Wood] = 5;

        assert!(total.has(ResourceKind::Wood, 1));
        assert!(total.has(ResourceKind::Wood, 5));
        assert_eq!(false, total.has(ResourceKind::Wood, 15));
        assert_eq!(false, total.has(ResourceKind::Food, 1));
    }

    #[test]
    fn resource_total_add() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Wood] = 5;

        assert!(total.has(ResourceKind::Wood, 5));
        total.add(ResourceKind::Wood, 10);
        assert!(total.has(ResourceKind::Wood, 15));
    }

    #[test]
    fn resource_total_add_range() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Wood] = 5;
        total.add_range(&vec![
            ResourceAmount::init(ResourceKind::Wood, 10),
            ResourceAmount::init(ResourceKind::Knowledge, 10),
        ]);

        assert!(total.has(ResourceKind::Wood, 15));
        assert!(total.has(ResourceKind::Knowledge, 10));
    }

    #[test]
    fn resource_total_add_range_with_coefficient() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Wood] = 5;
        total.add_range_with_coefficient(
            &vec![ResourceAmount::init(ResourceKind::Wood, 10), ResourceAmount::init(ResourceKind::Knowledge, 10)],
            2.5,
        );

        assert!(total.has(ResourceKind::Wood, 30));
        assert!(total.has(ResourceKind::Knowledge, 25));
    }

    #[test]
    fn resource_total_remove() {
        let mut total = ResourceTotal::init();
        total[ResourceKind::Wood] = 5;
        assert!(total.has(ResourceKind::Wood, 5));
        total.remove(ResourceKind::Wood, 4);
        assert!(total.has(ResourceKind::Wood, 1));
    }

    #[test]
    fn resource_combine() {
        let mut a = ResourceTotal::init();
        a[ResourceKind::Food] = 5;
        a[ResourceKind::Wood] = 5;

        let mut b = ResourceTotal::init();
        b[ResourceKind::Food] = 5;
        a.combine(&b);

        assert_eq!(10, a[ResourceKind::Food]);
        assert_eq!(5, a[ResourceKind::Wood]);
    }
}
