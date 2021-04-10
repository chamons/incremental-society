mod constants;
mod ecs;
mod identifier;
mod jobs;
mod needs;
mod paths;
mod pops;
mod resources;
mod util;
mod world;

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub mod prelude {
    pub use super::constants::*;
    pub use super::ecs::*;
    pub use super::identifier::*;
    pub use super::jobs::*;
    pub use super::needs::*;
    pub use super::paths::*;
    pub use super::pops::*;
    pub use super::resources::*;
    pub use super::util::*;
    pub use super::world::*;
}
