use std::collections::HashMap;

use super::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Job {
    name: String,
    resources: HashMap<String, u32>,
}

pub struct JobResource {
    pub jobs: Vec<Job>,
}

impl JobResource {
    pub fn new(jobs: Vec<Job>) -> JobResource {
        JobResource { jobs }
    }
}

pub fn load_jobs() -> JobResource {
    let input = read_string("data", "jobs.json");

    JobResource::new(serde_json::from_str(&input).unwrap())
}
