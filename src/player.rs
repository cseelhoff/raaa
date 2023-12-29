use serde::Deserialize;

#[derive(Default, Debug, Deserialize)]
pub struct Player {
    //moneyIndex: u16,
    //reserved_money_index: u16,
    pub name: &'static str,
    pub team: u8,
    pub is_human: bool,
    pub capital: &'static str,
    pub capital_index: usize,
    pub allies: Vec<bool>,
    //pub unit_stack_indicies: Vec<u16>,

    pub index: u8,
    //data: Data,

    //observables
    pub money: u8,
    reserved_money: u8
}

impl Player {
    pub fn new(name: &'static str, team: u8, index: u8) -> Self {
        Self {
            name,
            team,
            capital: "",
            capital_index: 0,
            is_human: false,
            allies: Vec::new(),
            //unit_stack_indicies: Vec::new(),
            index,
            money: 0,
            reserved_money: 0
        }
    }
}
