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

#[derive(Default, Debug, PartialEq)]
pub struct Data {
    owner_id: u8,
    is_owned: bool,
    is_ally_owned: bool,
    factory_max: u8,
    factory_health: u8,
    construction_remaining: u8,
    recently_conquered: bool,
    my_unit_stacks: crate::unit_stack::UnitStacks,
    enemy_unit_stacks: [crate::unit_stack::UnitStacks; 3],
    ally_unit_stacks: [crate::unit_stack::UnitStacks; 2],
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

    pub(crate) fn set_owner(&self, owner_id: u8, current_player: &Player, game_data: &mut GameData) {
        game_data.territories[self.index].owner_id = owner_id;
        self.update_ownership_flags(current_player, game_data)
    }

    fn update_ownership_flags(&self, current_player: &Player, game_data: &mut GameData) { 
        let territory_data: Data = game_data.territories[self.index];
        let current_owner_id:u8 = territory_data.owner_id;
        if current_owner_id == current_player.index {
            territory_data.is_owned = true;
            territory_data.is_enemy_owned = false;
            territory_data.is_ally_owned = true;
        } else {
            territory_data.is_owned = false;
            let ally_owned: bool = current_player.ally_indicies.contains(&current_owner_id);            
            territory_data.is_enemy_owned = !ally_owned;
            territory_data.is_ally_owned = ally_owned;
        }
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