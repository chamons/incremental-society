use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::*;

#[derive(Component, Serialize, Deserialize, Clone, Default)]
pub struct PopComponent {}
