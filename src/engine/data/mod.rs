#[allow(clippy::module_inception)]
#[cfg(not(test))]
mod data;
#[cfg(not(test))]
use data::*;

#[cfg(test)]
mod test;
#[cfg(test)]
use test::*;

mod interface;
pub use interface::*;
