use std::fmt;

use crate::connection::Connection;
use crate::engine::GameData;
use crate::player::Player;

#[derive(Debug)]
pub(crate) struct Territory {
    pub name: &'static str,
    land_value: u8,
    is_water: bool,
    original_owner_id: usize,
    pub connections: Vec<Connection>,
    water_connections: Vec<Connection>,
    land_connections: Vec<Connection>,
    adjacent_water_territories: Vec<Territory>,
    adjacent_land_territories: Vec<Territory>,
    adjacent_air_territories: Vec<Territory>,
    buildable_territories: Vec<Territory>,
    index: usize,
}

impl fmt::Display for Territory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct Data {
    pub owner_id: usize,
    is_owned: bool,
    is_ally_owned: bool,
    factory_max: u8,
    factory_health: u8,
    construction_remaining: u8,
    recently_conquered: bool,
    my_moved_unit_stacks: [u16; 1], // 1 unit status type
    alternating_team_unit_stacks: [[u16; 2]; 1], // 2 total players, 1 unit status type
    //current_player, enemy1, ally1, enemy2, ect...
}

impl Territory {
    pub fn new(name: &'static str, land_value: u8, original_owner_id: usize, index: usize) -> Self {
        Self {
            name,
            land_value,
            is_water: false,
            original_owner_id,
            connections: Vec::new(),
            water_connections: Vec::new(),
            land_connections: Vec::new(),
            adjacent_water_territories: Vec::new(),
            adjacent_land_territories: Vec::new(),
            adjacent_air_territories: Vec::new(),
            buildable_territories: Vec::new(),
            index,
        }
    }

    pub(crate) fn build_factory(&self, game_data: &mut GameData) {
        game_data.territories[self.index].factory_max = self.land_value;
        game_data.territories[self.index].factory_health = self.land_value;
        game_data.territories[self.index].construction_remaining = self.land_value;
    }

    pub(crate) fn set_owner(&self, owner_id: usize, current_player: &Player, game_data: &mut GameData) {
        game_data.territories[self.index].owner_id = owner_id;
        self.update_ownership_flags(current_player, game_data)
    }

    fn update_ownership_flags(&self, current_player: &Player, game_data: &mut GameData) {        
        let territory_data: &mut Data = &mut game_data.territories[self.index];
        let current_owner_id:usize = territory_data.owner_id;
        territory_data.is_owned = current_owner_id == current_player.index;
        territory_data.is_ally_owned = current_player.ally_indicies.contains(&current_owner_id);
    }

}
pub(crate) fn create_territories() -> Vec<Territory> {
    let mut territories: Vec<Territory> = Vec::new();
    add_territory(&mut territories, "Russia", 8, 0);
    add_territory(&mut territories, "Germany", 8, 1);
    territories
}

fn add_territory(territories: &mut Vec<Territory>, name: &'static str, land_value: u8, original_owner_id: usize) {
    territories.push(Territory::new(
        name,
        land_value,
        original_owner_id,
        territories.len(),
    ));
}