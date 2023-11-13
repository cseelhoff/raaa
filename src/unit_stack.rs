use crate::unit_status::UnitStatus;

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

pub fn create_unit_stacks_for_player(player:Player, unmoved_unit_statuses: Vec<UnitStatus>, moved_unit_statuses: Vec<UnitStatus>) -> (Vec<UnitStack>, Vec<UnitStack>) {
    let mut unmoved_unit_stacks: Vec<UnitStack> = Vec::new();
    for unmoved_unit_status in &unmoved_unit_statuses {
        unmoved_unit_stacks.push(UnitStack::new(
            unmoved_unit_status,
            player,
            0,
            None,
            None,
            None
        ));
    }
    let mut moved_unit_stacks: Vec<UnitStack> = Vec::new();
    for moved_unit_status in &moved_unit_statuses {
        moved_unit_stacks.push(UnitStack::new(
            moved_unit_status,
            player,
            0,
            None,
            None,
            None
        ));
    }
    for unmoved_unit_stack in unmoved_unit_stacks:
        for moved_unit_stack in moved_unit_stacks:
            if unmoved_unit_stack.unit_status == moved_unit_stack.unit_status and  {
                unmoved_unit_stack.unitStatusAfterMove = moved_unit_stack;
                moved_unit_stack.unitStatusAfterMove = unmoved_unit_stack;
            }
}