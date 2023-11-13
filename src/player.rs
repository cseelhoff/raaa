use serde::Deserialize;

#[derive(Default, Debug, PartialEq)]
pub struct Player {
    //moneyIndex: u16,
    //reserved_money_index: u16,
    pub name: &'static str,
    pub team: u8,
    pub is_human: bool,
    pub capital_index: usize,
    pub allies: Vec<bool>,
    //pub unit_stack_indicies: Vec<u16>,

    pub index: usize,
    //data: Data,

    //observables
    pub money: u8,
    reserved_money: u8
}

impl Player {
    pub fn new(name: &'static str, team: u8, capital_index: usize, index: usize) -> Self {
        Self {
            name,
            team,
            capital_index,
            is_human: false,
            allies: Vec::new(),
            //unit_stack_indicies: Vec::new(),
            index,
            money: 0,
            reserved_money: 0
        }
    }

    pub fn new_human(name: &'static str, team: u8, capital_index: usize, index: usize) -> Self {
        Self {
            name,
            team,
            capital_index,
            is_human: true,
            allies: Vec::new(),
            //unit_stack_indicies: Vec::new(),
            index,
            money: 0,
            reserved_money: 0
        }
    }

}
