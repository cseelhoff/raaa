use crate::player::Player;

#[derive(Default, Debug, PartialEq)]
pub struct UnitType {
    pub player_index: u8,
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

pub fn create_unit_types(players: Vec<Player>) -> Vec<UnitType> {
    let mut unit_types: Vec<UnitType> = Vec::new();
    let mut i:u8 = 0;
    while usize::from(i) < players.len() {
        //unit_types.push(UnitType {
        //    player_index: i, name: "Infantry", attack: 1, defense: 2, max_moves: 1, max_hits: 1, cost: 3, max_supportable: 1, weight: 2, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Artillery", attack: 2, defense: 2, max_moves: 1, max_hits: 1, cost: 4, max_supported: 1, weight: 3, ..Default::default()});
        /*
        unit_types.push(UnitType {
            player_index: i, name: "Armour", attack: 3, defense: 3, max_moves: 2, max_hits: 1, cost: 6, weight: 2, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Fighter", attack: 3, defense: 4, max_moves: 4, max_hits: 1, cost: 10, weight: 1, is_air: true, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Bomber", attack: 4, defense: 1, max_moves: 6, max_hits: 1, cost: 15, weight: 3, is_air: true, bombs: 6, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Submarine", attack: 2, defense: 1, max_moves: 2, max_hits: 1, cost: 6, is_water: true, is_sub: true, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Carrier", attack: 1, defense: 3, max_moves: 2, max_hits: 1, cost: 16, is_water: true, max_air: 2, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Destroyer", attack: 2, defense: 2, max_moves: 2, max_hits: 1, cost: 8, is_water: true, is_anti_sub: true, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Battleship", attack: 4, defense: 4, max_moves: 2, max_hits: 2, cost: 20, is_water: true, bombard: 4, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Crusier", attack: 3, defense: 3, max_moves: 2, max_hits: 1, cost: 12, is_water: true, bombard: 3, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "Transport", attack: 0, defense: 0, max_moves: 2, max_hits: 1, cost: 7, is_water: true, max_land: 5, ..Default::default()});
        unit_types.push(UnitType {
            player_index: i, name: "AntiAir", attack: 0, defense: 0, max_moves: 1, max_hits: 1, cost: 5, weight: 3, aa_shots: 3, ..Default::default()});
        */
        i = i + 1;
    }
    unit_types
}
