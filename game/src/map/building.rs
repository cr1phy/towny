// TODO: kinds of building
#[derive(Debug, Default)]
pub enum BuildingKind {
    #[default]
    Residential,
    Public,
    Industrial,
    Agricultural,
}

#[derive(Debug, Default)]
pub struct Building {
    id: u64,
    name: String,
    kind: BuildingKind,

    level: u64,
    price: f64,

    earnings_per_hour: f64,
}
