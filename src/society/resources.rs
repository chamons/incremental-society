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

    pub fn has(&self, kind: &str, amount: u32) -> bool {
        self.get(kind) >= amount
    }

    pub fn apply(&mut self, kind: &str, delta: i32) {
        if delta >= 0 {
            self.add(kind, delta as u32);
        } else {
            self.remove(kind, delta.abs() as u32);
        }
    }

    pub fn add(&mut self, kind: &str, amount: u32) {
        let value = self.storage.entry(kind.to_string()).or_insert(0);
        *value = *value + amount;
    }

    pub fn remove(&mut self, kind: &str, amount: u32) {
        let value = self.storage.entry(kind.to_string()).or_insert(0);
        if *value < amount {
            *value = 0;
        } else {
            *value = *value - amount;
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
    fn has() {
        let mut resources = Resources::new();
        assert!(!resources.has("Food", 10));
        resources.add("Food", 10);
        assert!(resources.has("Food", 10));
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
        resources.remove("Food", 10);
        assert_eq!(0, resources.get("Food"));
        resources.remove("Food", 10);
        assert_eq!(0, resources.get("Food"));
    }

    #[test]
    fn apply() {
        let mut resources = Resources::new();
        resources.add("Food", 10);
        resources.apply("Food", -10);
        assert_eq!(0, resources.get("Food"));
        resources.apply("Food", 30);
        assert_eq!(30, resources.get("Food"));
    }

    #[test]
    fn remove_nonexistent() {
        let mut resources = Resources::new();
        resources.remove("Food", 10);
        assert_eq!(0, resources.get("Food"));
    }
}
