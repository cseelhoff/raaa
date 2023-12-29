use crate::unit_type::UnitType;
use crate::player::Player;
use crate::connection::Connection;
use crate::territory::Territory;
use crate::unit_health::UnitHealth;
use crate::unit_health::create_unit_healths;
use crate::unit_status::UnitStatus;
use crate::unit_status::create_unit_statuses;
use crate::inactive_unit_stack::InactiveUnitStack;

use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;
use std::collections::HashMap;

const PLAYER_COUNT: usize = 2;
const TERRITORY_COUNT: usize = 2;
const UNIT_HEALTH_COUNT: usize = 2;

//#[derive(Default, Debug, PartialEq)]
//pub(crate) struct GameData {
//    pub(crate) players_money: [u8; PLAYER_COUNT],
//    pub(crate) territories: [crate::territory::Data; TERRITORY_COUNT]
//}
pub(crate) fn initialize() {

    //let mut game_data: GameData = GameData {..Default::default() };

    //load objects from json file "unittypes.json"    
    let unit_types: Vec<UnitType> = load_unit_types();
    let unit_healths: Vec<UnitHealth> = create_unit_healths(&unit_types);
    let moved_unit_statuses: Vec<UnitStatus> = create_unit_statuses(&unit_types);  
    
    let mut player_lookup: HashMap<&str, &Player> = HashMap::new();
    let players: Vec<Player> = load_players(&mut player_lookup);
    
    let mut territory_lookup: HashMap<&str, &Territory> = HashMap::new();
    let territories: Vec<Territory> = load_territories(&mut territory_lookup, player_lookup);

    let graveyard = InactiveUnitStack::new(
        None,
        0
    );

    create_inactive_armies(&territories, players, unit_healths, graveyard);

    let mut connections: Vec<Connection> = load_connections(territory_lookup);

    let mut current_turn: usize = 0;
    let mut current_player: &Player = &players[current_turn];
    
    let mut turn_count: u16 = 0;
    while is_game_over(territories, players) == false && turn_count < 10 {
        current_player = &players[current_turn];
        if current_player.is_human {
            print_game_status(&players, &territories, &current_player);
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

fn load_unit_types() -> Vec<UnitType> {
    let file = File::open("unit_types.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let unit_types: Vec<UnitType> = from_reader(reader).expect("Could not deserialize JSON");
    return unit_types
}

fn load_players(player_lookup: &mut HashMap<&str, &Player>) -> Vec<Player> {
    let file = File::open("players.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let players: Vec<Player> = from_reader(reader).expect("Could not deserialize JSON");
    for player in &players {
        player_lookup.insert(player.name, player);
    }
    players
}

fn load_territories(territory_lookup: &mut HashMap<&str, &Territory>, player_lookup: HashMap<&str, &Player>) -> Vec<Territory> {
    let file = File::open("territories.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let territories: Vec<Territory> = from_reader(reader).expect("Could not deserialize JSON");
    for territory in &territories {
        territory.owner_id = player_lookup.get(&territory.owner).unwrap().index;
        territory_lookup.insert(territory.name, territory);
    }
    for player in player_lookup.values() {
        player.capital_index = territory_lookup.get(player.capital).unwrap().index;
    }
    return territories
}

fn load_connections(territory_lookup: HashMap<&str, &Territory>) -> Vec<Connection> {
    let file = File::open("connections.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let connections: Vec<Connection> = from_reader(reader).expect("Could not deserialize JSON");
    return connections
}

fn create_inactive_armies(territories: &[Territory], players: Vec<Player>, unit_healths: Vec<UnitHealth>, graveyard: InactiveUnitStack) {
    for territory in territories {
        for player in players {
            let mut inactive_armies_for_player: [[InactiveUnitStack; UNIT_HEALTH_COUNT]; PLAYER_COUNT] = [[InactiveUnitStack::default(); UNIT_HEALTH_COUNT]; PLAYER_COUNT];
            let mut last_inactive_army_for_player = graveyard;
            let unit_health_index: u8 = 0;
            for (unit_health_index, unit_health) in unit_healths.iter().enumerate() {
                let mut inactive_army_for_player = InactiveUnitStack::new(
                    unit_health.unit_type,
                    unit_health.hits_remaining,
                    graveyard
                );
                if unit_health.hits_remaining > 1 {
                    inactive_army_for_player.unitStatusAfterHit = last_inactive_army_for_player;
                }
                inactive_armies_for_player[player.index][unit_health_index] = inactive_army_for_player;
                last_inactive_army_for_player = Some(Box::new(inactive_army_for_player));
            }      
        }
    }
}


fn is_game_over(territories: Vec<Territory>, players: Vec<Player>) -> bool {
    players.iter().any(|player| territories[player.capital_index].owner_id != player.index)
}

fn print_game_status(players: &[Player], territories: &[Territory], current_player: &Player) {
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
