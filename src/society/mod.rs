mod ecs;
mod pops;
mod resources;
mod world;

#[macro_export]
macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

pub mod prelude {
    pub use super::ecs::*;
    pub use super::pops::*;
    pub use super::resources::*;
    pub use super::world::*;
}
