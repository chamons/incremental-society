use std::collections::HashMap;

pub struct Resources {
    storage: HashMap<String, u32>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources { storage: HashMap::new() }
    }

    pub fn kinds<'a>(&'a self) -> Box<dyn Iterator<Item = &String> + 'a> {
        Box::new(self.storage.keys())
    }

    pub fn get(&self, kind: &str) -> u32 {
        *self.storage.get(kind).unwrap_or(&0)
    }

    pub fn add(&mut self, kind: &str, amount: u32) {
        let value = self.storage.entry(kind.to_string()).or_insert(0);
        *value = *value + amount;
    }

    pub fn remove(&mut self, kind: &str, amount: u32) -> bool {
        let value = self.storage.entry(kind.to_string()).or_insert(0);
        if *value < amount {
            *value = 0;
            false
        } else {
            *value = *value - amount;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Resources;

    #[test]
    fn kinds() {
        let mut resources = Resources::new();
        assert_eq!(0, resources.kinds().count());
        resources.add("Food", 10);
        assert_eq!(1, resources.kinds().count());
        assert_eq!("Food", resources.kinds().next().unwrap());
    }

    #[test]
    fn get() {
        let mut resources = Resources::new();
        assert_eq!(0, resources.get("Food"));
        resources.add("Food", 10);
        assert_eq!(10, resources.get("Food"));
    }

    #[test]
    fn add() {
        let mut resources = Resources::new();
        resources.add("Food", 10);
        resources.add("Food", 10);
        resources.add("Food", 10);
        assert_eq!(30, resources.get("Food"));
    }

    #[test]
    fn remove() {
        let mut resources = Resources::new();
        resources.add("Food", 10);
        assert!(resources.remove("Food", 10));
        assert_eq!(0, resources.get("Food"));
        assert!(!resources.remove("Food", 10));
        assert_eq!(0, resources.get("Food"));
    }

    #[test]
    fn remove_nonexistent() {
        let mut resources = Resources::new();
        assert!(!resources.remove("Food", 10));
        assert_eq!(0, resources.get("Food"));
    }
}
