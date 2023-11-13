use crate::unit_status::UnitStatus;

pub struct UnitStack {
    pub(crate) unit_status: UnitStatus,
    pub(crate) owner: Player,
    pub(crate) unit_quantity: u8,
    pub(crate) unitStatusAfterMove: UnitStack, // this unit status changes to this after it moves (decrement one status quantity, increase another)
    pub(crate) unitStatusAfterHit: UnitStack,
    pub(crate) unitStatusAfterTurn: UnitStack,
}

pub fn create_unit_stacks(unmoved_unit_statuses: Vec<UnitStatus>, moved_unit_statuses: Vec<UnitStatus>) -> (Vec<UnitStack>, Vec<UnitStack>) {

}