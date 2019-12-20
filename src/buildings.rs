use crate::conversion::*;

#[derive(Debug)]
pub struct Building<'a> {
    pub name: &'a str,
    pub conversions: Vec<Conversion<'a>>,
}

impl<'a> Building<'a> {
    pub fn init_single(name: &'a str, conversion: Conversion<'a>) -> Building<'a> {
        Building::init(name, vec![conversion])
    }

    pub fn init(name: &'a str, conversions: Vec<Conversion<'a>>) -> Building<'a> {
        Building {
            name,
            conversions: conversions,
        }
    }
}
