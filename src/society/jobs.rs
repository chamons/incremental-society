use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specs::prelude::*;

use super::prelude::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct Job {
    pub name: String,
    pub resources: HashMap<String, u32>,
}

impl Job {
    #[cfg(test)]
    pub fn new_single(name: &str, resource: &str, amount: u32) -> Job {
        Job {
            name: name.to_string(),
            resources: [(resource.to_string(), amount)].iter().cloned().collect(),
        }
    }
}

pub struct JobLibrary {
    jobs: HashMap<String, Job>,
}

impl JobLibrary {
    pub fn load() -> JobLibrary {
        let input = read_string("data", "jobs.json");

        let jobs: Vec<Job> = serde_json::from_str(&input).unwrap();
        let jobs: HashMap<String, Job> = jobs.iter().map(|j| (j.name.to_owned(), j.clone())).collect();

        JobLibrary { jobs }
    }

    pub fn get(&self, job: &str) -> &Job {
        self.jobs.get(job).expect(&format!("Unable to find job {}", job))
    }

    #[cfg(test)]
    pub fn add_job(&mut self, job: Job) {
        self.jobs.insert(job.name.to_owned(), job);
    }
}

pub fn tick_jobs(ecs: &mut World) {
    let default_job = ecs.get_string_constant("DEFAULT_JOB");

    let mut total_jobs = HashMap::new();

    let pops = ecs.read_storage::<PopComponent>();
    for pop in (&pops).join() {
        let job = pop.job.as_ref().unwrap_or_else(|| &default_job);
        total_jobs.entry(job.to_string()).and_modify(|j| *j += 1).or_insert(1);
    }

    let mut resources = ecs.write_resource::<Resources>();
    let job_library = ecs.read_resource::<JobLibrary>();
    for (job, pops_working) in total_jobs {
        for (resource, amount) in &job_library.get(&job).resources {
            resources.add(resource, pops_working * amount);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn setup_job_world() -> World {
        let ecs = register_world();
        ecs.write_resource::<JobLibrary>().add_job(Job::new_single("TestJob", "Food", 5));
        ecs.write_resource::<JobLibrary>().add_job(Job::new_single("TestOtherJob", "Wood", 10));

        ecs.write_resource::<ConstantLibrary>().set("DEFAULT_JOB", Value::String("TestJob".to_string()));
        ecs
    }

    #[test]
    fn tick_assigned_job() {
        let mut ecs = setup_job_world();

        for job in &["TestOtherJob", "TestOtherJob", "TestJob"] {
            let id = ecs.next_id();
            let mut pop = PopComponent::new();
            pop.job = Some(job.to_string());
            ecs.create_entity().with(pop).with(id).build();
        }
        tick_jobs(&mut ecs);

        assert_eq!(5, ecs.read_resource::<Resources>().get("Food"));
        assert_eq!(20, ecs.read_resource::<Resources>().get("Wood"));
    }

    #[test]
    fn tick_no_job() {
        let mut ecs = setup_job_world();

        for _ in 0..2 {
            let id = ecs.next_id();
            ecs.create_entity().with(PopComponent::new()).with(id).build();
        }
        tick_jobs(&mut ecs);

        assert_eq!(10, ecs.read_resource::<Resources>().get("Food"));
        assert_eq!(0, ecs.read_resource::<Resources>().get("Wood"));
    }
}
