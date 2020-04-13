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

#[cfg(test)]
pub mod tests {
    pub use crate::data::get_building as get_test_building;
    pub use crate::data::get_edict as get_test_edict;
    pub use crate::data::get_research as get_test_research;
    pub use crate::data::get_upgrade as get_test_upgrade;

    pub fn assert_is_none<T>(item: Option<T>) {
        assert_eq!(true, matches!(item, None));
    }

    pub fn assert_is_some<T>(item: Option<T>) {
        assert_eq!(true, matches!(item, Some(_)));
    }
}
