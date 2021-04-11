use std::collections::HashMap;

use serde_json::Value;
use specs::prelude::*;

use super::prelude::*;

#[derive(Default)]
pub struct ConstantLibrary {
    constants: HashMap<String, Value>,
}

impl ConstantLibrary {
    pub fn load() -> ConstantLibrary {
        let input = read_string("data", "constants.json");

        ConstantLibrary {
            constants: serde_json::from_str(&input).unwrap(),
        }
    }

    pub fn get_u32(&self, key: &str) -> u32 {
        self.constants.get(key).unwrap().as_u64().unwrap() as u32
    }

    pub fn get_f64(&self, key: &str) -> f64 {
        self.constants.get(key).unwrap().as_f64().unwrap()
    }

    pub fn get_string(&self, key: &str) -> String {
        self.constants.get(key).unwrap().as_str().unwrap().to_string()
    }

    #[cfg(test)]
    pub fn set(&mut self, key: &str, value: Value) {
        self.constants.insert(key.to_string(), value);
    }
}

pub trait EasyConstants {
    fn get_constant(&self, name: &str) -> u32;
    fn get_float_constant(&self, name: &str) -> f64;
    fn get_string_constant(&self, name: &str) -> String;
}

impl EasyConstants for World {
    fn get_constant(&self, name: &str) -> u32 {
        self.read_resource::<ConstantLibrary>().get_u32(name)
    }

    fn get_float_constant(&self, name: &str) -> f64 {
        self.read_resource::<ConstantLibrary>().get_f64(name)
    }

    fn get_string_constant(&self, name: &str) -> String {
        self.read_resource::<ConstantLibrary>().get_string(name)
    }
}