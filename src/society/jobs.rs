use std::collections::HashMap;

use super::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Job {
    name: String,
    resources: HashMap<String, u32>,
}

pub struct JobLibrary {
    pub jobs: Vec<Job>,
}

impl JobLibrary {
    pub fn load() -> JobLibrary {
        let input = read_string("data", "jobs.json");

        JobLibrary {
            jobs: serde_json::from_str(&input).unwrap(),
        }
    }
}
