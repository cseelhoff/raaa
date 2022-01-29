use crate::engine::GameData;

pub struct Player {
    //moneyIndex: u16,
    //reserved_money_index: u16,
    pub name: &'static str,
    pub team: u8,
    pub is_human: bool,
    pub capital_index: usize,
    pub ally_indicies: Vec<usize>,
    pub enemy_indicies: Vec<usize>,
    //pub unit_stack_indicies: Vec<u16>,

    pub index: usize,
    //data: Data,
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Data {
    money: u8,
    reserved_money: u8
}

impl Player {
    pub fn new(name: &'static str, team: u8, capital_index: usize, index: usize) -> Self {
        Self {
            name,
            team,
            capital_index,
            is_human: false,
            ally_indicies: Vec::new(),
            enemy_indicies: Vec::new(),
            //unit_stack_indicies: Vec::new(),
            index
        }
    }

    pub fn new_human(name: &'static str, team: u8, capital_index: usize, index: usize) -> Self {
        Self {
            name,
            team,
            capital_index,
            is_human: true,
            ally_indicies: Vec::new(),
            enemy_indicies: Vec::new(),
            //unit_stack_indicies: Vec::new(),
            index
        }
    }
    
    pub(crate) fn set_money(&self, game_data: &mut GameData, money: u8) {
        game_data.players[self.index] = money;
    }

    pub(crate) fn owns_captial(&self, game_data: &GameData) -> bool {
        game_data.territories[self.capital_index].owner_id == self.index
    }
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
    players
}