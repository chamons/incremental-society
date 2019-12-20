use crate::buildings::*;

#[derive(Debug)]
pub struct Region<'a> {
    pub name: &'a str,
    pub buildings: Vec<Building<'a>>,
}

impl<'a> Region<'a> {
    pub fn init(name: &'a str, buildings: Vec<Building<'a>>) -> Region<'a> {
        Region { name, buildings }
    }
}
