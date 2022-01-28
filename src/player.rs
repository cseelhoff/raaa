pub struct Player {
    //moneyIndex: u16,
    //reserved_money_index: u16,
    pub name: &'static str,
    pub team: u8,
    pub is_human: bool,
    pub capital_index: u8,
    pub ally_indicies: Vec<u8>,
    pub enemy_indicies: Vec<u8>,
    pub unit_stack_indicies: Vec<u16>,

    pub index: u8,
    //data: Data,
}

#[derive(Default, Debug, PartialEq)]
pub(crate) struct Data {
    money: u8,
    reserved_money: u8
}

impl Player {
    pub fn new(name: &'static str, team: u8, capital_index: u8, index: u8) -> Self {
        Self {
            name,
            team,
            capital_index,
            is_human: false,
            ally_indicies: Vec::new(),
            enemy_indicies: Vec::new(),
            unit_stack_indicies: Vec::new(),
            index
        }
    }
}

pub fn create_players() -> Vec<Player> {
    let mut players: Vec<Player> = Vec::new();
    players.push(Player::new(
        "Russia",
        1,
        0,
        players.len() as u8
    ));
    players.push(Player::new(
        "Germany",
        2,
        1,
        players.len() as u8
    ));
    players
}