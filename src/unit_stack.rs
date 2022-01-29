use crate::unit_type::{UnitType, UnitTypes};

pub struct UnitStack {
    pub(crate) unit_type: UnitType,
    pub(crate) moves_remaining: u8,
    pub(crate) hits_remaining: u8,
    pub(crate) quantity_index_after_move_index: usize,
    pub(crate) quantity_index_after_hit_index: usize,
    pub(crate) quantity_index_after_turn_index: usize,
}

pub fn create_unit_stacks(unit_types: &UnitTypes) -> (Vec<UnitStack>, Vec<UnitStack>) {
    let unmoved_unit_stacks = vec![
        UnitStack {
            unit_type: unit_types.artillery,
            moves_remaining: 1,
            hits_remaining: 1,
            quantity_index_after_move_index: 0,
            quantity_index_after_hit_index: 0,
            quantity_index_after_turn_index: 0,
        }
    ];
    let moved_unit_stacks: Vec<UnitStack> = vec![
        UnitStack {
            unit_type: unit_types.artillery,
            moves_remaining: 0,
            hits_remaining: 1,
            quantity_index_after_move_index: 0,
            quantity_index_after_hit_index: 0,
            quantity_index_after_turn_index: 0,
        }
    ];

    (unmoved_unit_stacks, moved_unit_stacks)
}
