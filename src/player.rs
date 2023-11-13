use serde::Deserialize;

use crate::territory::Territory;

#[derive(Default, Debug, PartialEq)]
pub struct Player {
    //moneyIndex: u16,
    //reserved_money_index: u16,
    pub name: &'static str,
    pub team: u8,
    pub is_human: bool,
    pub capital: &'static str,
    pub capital_territory: Territory,
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
            capital: "",
            capital_territory: None,
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
            capital: "",
            capital_territory: None,
            is_human: true,
            allies: Vec::new(),
            //unit_stack_indicies: Vec::new(),
            index,
            money: 0,
            reserved_money: 0
        }
    }

}
