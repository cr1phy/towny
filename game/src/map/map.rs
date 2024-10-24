use std::collections::HashSet;

use super::{building::Building, coordinates::Coordinates};

#[derive(Debug, Default)]
pub struct Map {
    id: u64,
    name: String,
    size: Vec<u64>,
    buildings: HashSet<Coordinates, Building>,
    _display: String,
}

impl Map {
    pub fn generate() -> Self {
        Self {
            id: todo!(),
            name: todo!(),
            size: todo!(),
            buildings: todo!(),
            _display: todo!(),
        }
    }
}
