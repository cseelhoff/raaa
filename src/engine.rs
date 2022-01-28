use crate::{unit_type::UnitType, player::Player, unit_type::create_unit_types, player::create_players};
use crate::connection::create_connections;
use crate::territory::{Territory, create_territories};

#[derive(Default, Debug, PartialEq)]
pub(crate) struct GameData {
    pub(crate) players: [crate::player::Data; 2],
    pub(crate) territories: [crate::territory::Data; 2]
}

pub(crate) fn initialize() {
    //static
    //let verbose: bool = false;
    //let mut game_data = [u8; 32000];
    //let mut data_size: u16 = 0;

    let mut game_data: GameData = GameData {..Default::default() };    
    let mut territories: Vec<Territory> = create_territories();
    create_connections(&mut territories);
    let mut players: Vec<Player> = create_players();    
    let mut unit_types: Vec<UnitType> = create_unit_types(players);
    println!("{}", &territories[0].name);

    let mut current_turn = 0;
    let mut current_player = &players[current_turn];

    territories[0].build_factory(&mut game_data);
    territories[1].build_factory(&mut game_data);

    territories[0].set_owner(0, &mut game_data);
    territories[0].set_ownership_flags(current_player, &mut game_data);
    

    

}