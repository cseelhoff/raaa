/*
use crate::{unit_status::UnitStatus, player::Player};

pub struct UnitStack {
    pub(crate) unit_status: UnitStatus,
    pub(crate) owner: Player,
    pub(crate) unit_quantity: u8,
    pub(crate) unitStatusAfterMove: UnitStack, // this unit status changes to this after it moves (decrement one status quantity, increase another)
    pub(crate) unitStatusAfterHit: UnitStack,
    pub(crate) unitStatusAfterTurn: UnitStack,
}

impl UnitStack {
    pub fn new(unit_status: &UnitStatus, owner: &Player, unit_quantity: u8, unitStatusAfterMove: &UnitStatus, unitStatusAfterHit: &UnitStatus, unitStatusAfterTurn: &UnitStatus) -> Self {
        Self {
            unit_status: unit_status.clone(),
            owner: owner.clone(),
            unit_quantity,
            unitStatusAfterMove: UnitStack::new(unitStatusAfterMove, owner, unit_quantity, unitStatusAfterMove, unitStatusAfterHit, unitStatusAfterTurn),
            unitStatusAfterHit: UnitStack::new(unitStatusAfterHit, owner, unit_quantity, unitStatusAfterMove, unitStatusAfterHit, unitStatusAfterTurn),
            unitStatusAfterTurn: UnitStack::new(unitStatusAfterTurn, owner, unit_quantity, unitStatusAfterMove, unitStatusAfterHit, unitStatusAfterTurn),
        }
    }
}

pub fn create_unit_stacks(players:Vec<Player>, unmoved_unit_statuses: Vec<UnitStatus>, moved_unit_statuses: Vec<UnitStatus>) -> (Vec<UnitStack>, Vec<UnitStack>) {
    let mut unmoved_unit_stacks: Vec<UnitStack> = Vec::new();
    let mut moved_unit_stacks: Vec<UnitStack> = Vec::new();
    for player in &players {

    }
    return (unmoved_unit_stacks, moved_unit_stacks)
}
*/