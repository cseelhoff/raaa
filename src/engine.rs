use crate::unit_stack::{UnitStack, create_unit_stacks};
use crate::unit_type::{UnitTypes, self};
use crate::{unit_type::UnitType, player::Player, unit_type::create_unit_types, player::create_players};
use crate::connection::create_connections;
use crate::territory::{Territory, create_territories};
use std::fmt::Write;

const PLAYER_COUNT: usize = 2;
const TERRITORY_COUNT: usize = 2;

#[derive(Default, Debug, PartialEq)]
pub(crate) struct GameData {
    pub(crate) players: [u8; PLAYER_COUNT],
    pub(crate) territories: [crate::territory::Data; TERRITORY_COUNT]
}
pub(crate) fn initialize() {

    let mut game_data: GameData = GameData {..Default::default() };
    let unit_types: UnitTypes = create_unit_types();
    let (unmoved_unit_stacks, moved_unit_stacks) = 
        create_unit_stacks(&unit_types);  
        
    let players: Vec<Player> = create_players();
    let mut territories: Vec<Territory> = create_territories();
    create_connections(&mut territories);
    players[0].set_money(&mut game_data, 10);
    players[1].set_money(&mut game_data, 10);
    territories[0].build_factory(&mut game_data);
    territories[1].build_factory(&mut game_data);

    let mut current_turn: usize = 0;
    let mut current_player: &Player = &players[current_turn];

    territories[0].set_owner(0, current_player, &mut game_data);
    territories[1].set_owner(1, current_player, &mut game_data);
    
    let mut turn_count = 0;
    while is_game_over(&players, &game_data) == false && turn_count < 10 {
        current_player = &players[current_turn];
        if current_player.is_human {
            print_game_status(&players, &territories, &current_player, &game_data);
        }
        move_land_units();
        move_transport_units();
        move_sea_units();
        move_fighter_units();
        move_bomber_units();
        resolve_sea_battles();
        unload_transports();
        bomb_factories();
        bombard_shores();
        fire_aa_guns();
        resolve_land_battles();
        land_air_units();
        move_aa_guns();
        reset_units_fully();
        buy_units();
        crash_air_units();
        collect_money();
        
        current_turn = (current_turn + 1) % PLAYER_COUNT;
        turn_count = turn_count + 1;
    }
    println!("game over");
}


fn is_game_over(players: &Vec<Player>, game_data: &GameData) -> bool {
    let mut player_index = 0;
    while player_index < PLAYER_COUNT {
        if players[player_index].owns_captial(&game_data) == false {
            return true;
        }
        player_index = player_index + 1
    }
    false
}

fn print_game_status(players: &[Player], territories: &[Territory], current_player: &Player, game_data: &GameData) {
    let mut status: String = "CurrentTurn: ".to_owned();
    status.push_str(current_player.name);
    println!("{}", status);

    let mut territory_index = 0;
    while territory_index < TERRITORY_COUNT {
        println!("{}", territories[territory_index]);
        territory_index = territory_index + 1;
    }
}

fn move_land_units() {
    //foreach unit stack that the player owns
    //loop through land units
    // 1. no movement - set moves to 0
    // 2. move 1 space, reduce movement, and ask again
    //   a. if boarding, set movement remaining to receiving transports movement remaining
    //   b. if not boarding ask:
    //     1. 1 unit
    //     2. 50% stack (rounded down)
    //     3. 100% stack (rounded down)
    // 3. (if adj to water and transports with moves still exist) wait
}

fn move_transport_units() {
    //---repeat until all transports done---
    //loop through all transports
    // 1. no movement
    // 2. move 1 space
    //loop through land units
    // 1. load transport
    //   a. set movement remaining to receiving transports movement remaining
    // 2. (if adj to water and transports with moves still exist) wait
    //---
}

fn move_sea_units() {
    //loop through all remaining sea units
    // 1. no movement - set moves to 0
    // 2. move 1 space, reduce movement, and ask again
}
    
fn move_fighter_units() {
    
    //loop through all fighter units (no kamakaze check yet)
    // 1. (if available) no more movement
    // 2. move 1 space, reduce movement, and ask again
    // crash fighter if unsavable
}

fn move_bomber_units() {
    
    //loop through all bomber units (no kamakaze check yet)
    // 1. (if available) no more movement - bomber mode
    // 2. (if available) no more movement - attack mode
    // 3. move 1 space, reduce movement, and ask again
    // crash bomber if unsavable
}

fn resolve_sea_battles() {
}

fn unload_transports() {
    
    //loop through all transports
    // 1. no unload
    // 2. unload
    //
}

fn fire_aa_guns() {
}

fn bombard_shores() {
}

fn resolve_land_battles() {
    //land combat
    // a. round with retreat option
}

fn bomb_factories() {
}

fn land_air_units() {
    
    //loop through all air units
    // 1. no movement - set moves to 0
    // 2. move 1 space, reduce movement, and ask again
    // crash air unit if unsavable
}

fn move_aa_guns() {
    
    //loop through all aa units
    // 1. no movement - set moves to 0
    // 2. move 1 space, reduce movement, and ask again
}

fn reset_units_fully() {
}

fn buy_units() {
}

fn crash_air_units() {
}

fn collect_money() {
}
