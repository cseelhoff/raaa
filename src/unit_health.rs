use crate::unit_type::UnitType;

pub struct UnitHealth {
    pub unit_type: UnitType,
    pub hits_remaining: u8
}

impl UnitHealth {
    pub fn new(unit_type: UnitType, hits_remaining: u8) -> Self {
        Self {
            unit_type,
            hits_remaining
        }
    }
}

pub fn create_unit_healths(unit_types: Vec<UnitType>) -> Vec<UnitHealth> {
    //This list of unit statuses is for storing unmoved units - this is more efficent, since player's waiting for their turn only have this type of unit status to worry about
    //loop through each unit_type, and create a unit status for each quantity index
    let mut inactive_unit_types: Vec<UnitHealth> = Vec::new();
    //let mut unmoved_unit_statuses_index = 0;
    for unit_type in unit_types {
        for hits_remaining in 1..=unit_type.max_hits {
            inactive_unit_types.push(
                UnitHealth::new(
                    unit_type,
                    hits_remaining
                )
            );
        }
    }
    return inactive_unit_types;
}
