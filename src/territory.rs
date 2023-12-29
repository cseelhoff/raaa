use std::fmt;
use serde::Deserialize;

use crate::connection::Connection;
use crate::player::Player;
use crate::inactive_unit_stack::InactiveUnitStack;
use crate::unit_health::UnitHealth;
use crate::unit_status::UnitStatus;

#[derive(Debug, Deserialize)]
pub(crate) struct Territory {
    pub name: &'static str,
    pub land_value: u8,
    is_water: bool,
    original_owner_id: u8,
    pub connections: Vec<Connection>,
    water_connections: Vec<Connection>,
    land_connections: Vec<Connection>,
    adjacent_water_territories: Vec<Territory>,
    adjacent_land_territories: Vec<Territory>,
    adjacent_air_territories: Vec<Territory>,
    buildable_territories: Vec<Territory>,
    pub index: usize,
    pub owner: &'static str,
    //observables:
    pub owner_id: u8,
    is_owned: bool,
    is_ally_owned: bool,
    pub factory_max: u8,
    factory_health: u8,
    pub construction_remaining: u8,
    recently_conquered: bool,
    my_unit_stacks: [u8; 1], // 1 unit status type - this includes units that are other than full move points remaining
    inactive_armies: [[InactiveUnitStack; 1]; 2]   // 2 total players, 1 unit status type - this includes only unit types with full move points remaining
    //current_player, enemy1, ally1, enemy2, ect...
}

impl fmt::Display for Territory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Territory {
    pub fn new(name: &'static str, land_value: u8, original_owner_id: u8, index: usize) -> Self {
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
            owner: "",
            owner_id: 0,
            is_owned: false,
            is_ally_owned: false,
            factory_max: 0,
            factory_health: 0,
            construction_remaining: 0,
            recently_conquered: false,
            my_unit_stacks: [1], // 1 unit status type
            inactive_armies: [[InactiveUnitStack{ unit_health: None, quantity: 0, stack_after_hit: None }; 1]; 2]   // 2 total players, 1 unit status type
        }
    }


    pub fn create_my_unit_stacks(&self, players: Vec<Player>, unit_statuses: Vec<UnitStatus>, graveyard: InactiveUnitStack) {
        
    }

    pub(crate) fn build_factory(&self) {
        self.factory_max = self.land_value;
        self.factory_health = self.land_value;
    }
    
    pub(crate) fn reset_factory(&self) {
        self.construction_remaining = self.factory_max;
    }
    
}

//fn update_ownership_flags(&self, current_player: &Player, game_data: &mut GameData) {        
//    let territory_data: &mut Data = &mut game_data.territories[self.index];
//    let current_owner_id:usize = territory_data.owner_id;
//    territory_data.is_owned = current_owner_id == current_player.index;
//    territory_data.is_ally_owned = current_player.allies[current_owner_id];
    //territory_data.is_ally_owned = current_player.ally_indicies.contains(&current_owner_id);
//}
