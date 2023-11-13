use serde::Deserialize;

#[derive(Default, Debug, PartialEq, Clone, Copy, Deserialize)]
pub struct UnitType {
    pub name: &'static str,
    pub attack: u8,
    pub defense: u8,
    pub max_moves: u8,
    pub max_hits: u8,
    pub cost: u8,
    pub max_supportable: u8,
    pub max_supported: u8,
    pub weight: u8,
    pub is_air: bool,
    pub bombs: u8,
    pub is_water: bool,
    pub max_land: u8,
    pub max_air: u8,
    pub is_sub: bool,
    pub is_anti_sub: bool,
    pub bombard: u8,
    pub aa_shots: u8,
}
