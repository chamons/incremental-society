use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Serialize, Deserialize, Clone, Default)]
pub struct PopComponent {
    job: Option<String>,
}

impl PopComponent {
    pub fn new() -> PopComponent {
        PopComponent { job: None }
    }
}
