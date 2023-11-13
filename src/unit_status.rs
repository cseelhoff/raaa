use crate::{unit_type::UnitType, player::Player};

pub struct UnitStatus {
    pub unit_type: UnitType,
    pub moves_remaining: u8,
    pub hits_remaining: u8
}

impl UnitStatus {
    pub fn new(unit_type: UnitType, moves_remaining: u8, hits_remaining: u8) -> Self {
        Self {
            unit_type,
            moves_remaining,
            hits_remaining,
        }
    }
}

pub fn create_unit_statuses(unit_types: Vec<UnitType>) -> (Vec<UnitStatus>, Vec<UnitStatus>) {
    //This list of unit statuses is for storing unmoved units - this is more efficent, since player's waiting for their turn only have this type of unit status to worry about
    //loop through each unit_type, and create a unit status for each quantity index
    let unmoved_unit_statuses: Vec<UnitStatus> = Vec::new();
    //let mut unmoved_unit_statuses_index = 0;
    for unit_type in &unit_types {
        for hits_remaining in 1..=unit_type.max_hits {
            unmoved_unit_statuses.push(UnitStatus {
                unit_type,
                moves_remaining: unit_type.moves,
                hits_remaining,
            });
        }
    }
    let moved_unit_statuses: Vec<UnitStatus> = Vec::new();
    for unit_type in &unit_types {
        for hits_remaining in 1..=unit_type.max_hits {
            for moves_remaining in 0..unit_type.moves {
                moved_unit_statuses.push(UnitStatus {
                    unit_type,
                    moves_remaining,
                    hits_remaining,
                });
            }
        }
    }
    
    return (unmoved_unit_statuses, moved_unit_statuses)
}
