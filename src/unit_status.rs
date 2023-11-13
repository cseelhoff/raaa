use crate::unit_health::UnitHealth;

pub struct UnitStatus {
    pub unit_health: UnitHealth,
    pub moves_remaining: u8,    
}

impl UnitStatus {
    pub fn new(unit_health: UnitHealth, moves_remaining: u8) -> Self {
        Self {
            unit_health,
            moves_remaining
        }
    }
}

pub fn create_unit_statuses(player: Player, unit_healths: Vec<UnitHealth>) -> Vec<UnitStatus> {
    //This list of unit statuses is for storing unmoved units - this is more efficent, since player's waiting for their turn only have this type of unit status to worry about
    //loop through each unit_type, and create a unit status for each quantity index
    let moved_unit_statuses: Vec<UnitStatus> = Vec::new();
    for unit_health in &unit_healths {
        for moves_remaining in 0..unit_type.moves {
            moved_unit_statuses.push(UnitStatus::new(
                unit_type,
                moves_remaining,
                hits_remaining
            ));
        }
    }
    
    return moved_unit_statuses
}
