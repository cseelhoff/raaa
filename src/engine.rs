use crate::unit_status::create_unit_statuses;
use crate::unit_type::UnitType;
use crate::player::Player;
use crate::connection::Connection;
use crate::territory::Territory;
use std::fmt::Write;
use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;

const PLAYER_COUNT: usize = 2;
const TERRITORY_COUNT: usize = 2;

//#[derive(Default, Debug, PartialEq)]
//pub(crate) struct GameData {
//    pub(crate) players_money: [u8; PLAYER_COUNT],
//    pub(crate) territories: [crate::territory::Data; TERRITORY_COUNT]
//}
pub(crate) fn initialize() {

    //let mut game_data: GameData = GameData {..Default::default() };

    //load objects from json file "unittypes.json"    
    let unit_types: Vec<UnitType> = load_unit_types();
    let (unmoved_unit_stacks, moved_unit_stacks) = create_unit_statuses(&unit_types);  
        
    let mut players: Vec<Player> = load_players();
    let mut territories: Vec<Territory> = load_territories();
    let mut connections: Vec<Connection> = load_connections();

    territories[0].build_factory();
    territories[1].build_factory();

    territories[0].reset_factory();
    territories[1].reset_factory();

    players[0].money = 10;
    players[1].money = 10;

    let mut current_turn: usize = 0;
    let mut current_player: &Player = &players[current_turn];

    territories[0].set_owner(0, current_player, &mut game_data);
    territories[1].set_owner(1, current_player, &mut game_data);
    
    let mut turn_count: u16 = 0;
    while is_game_over() == false && turn_count < 10 {
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

fn load_players() -> Vec<Player> {
    let file = File::open("players.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let players: Vec<Player> = from_reader(reader).expect("Could not deserialize JSON");
    return players
}

fn load_territories() -> Vec<Territory> {
    let file = File::open("territories.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let territories: Vec<Territory> = from_reader(reader).expect("Could not deserialize JSON");
    return territories
}

fn load_connections() -> Vec<Connection> {
    let file = File::open("connections.json").expect("Could not open file");
    let reader = BufReader::new(file);
    let connections: Vec<Connection> = from_reader(reader).expect("Could not deserialize JSON");
    return connections
}

pub fn create_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new_human(
        "Russia",
        1,
        0,
        players.len() as usize
    ));
    players.push(Player::new(
        "Germany",
        2,
        1,
        players.len() as usize
    ));
    players[0].allies.push(false);
    players[1].allies.push(false);
    return players
}

fn create_territories() -> Vec<Territory> {
    let mut territories: Vec<Territory> = Vec::new();
    territories.push(Territory::new(
        "Russia",
        8,
        get_player_index(players, "Russia"),
        territories.len(),
    ));
    territories.push(Territory::new(
        "Germany",
        8,
        get_player_index(players, "Germany"),
        territories.len(),
    ));
    territories
}

fn get_player_index(players: _, arg: &str) -> usize {
    players.iter().position(|player| player.name == arg).unwrap()
}

fn is_game_over() -> bool {
    (0..PLAYER_COUNT).any(|player_index| !player_owns_capital(player_index))
}

fn player_owns_capital(player_index: u8) -> bool {
    return territories[players[player_index].capital_index].owner_id == player_index
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
