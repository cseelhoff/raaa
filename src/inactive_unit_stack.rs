use crate::unit_type::UnitType;

pub struct InactiveUnitStack {
    pub unit_health: UnitHealth,
    pub quantity: u8,
    pub stack_after_hit: InactiveUnitStack
}

impl UnitHealth {
    pub fn new(unit_health: UnitHealth, quantity: u8) -> Self {
        Self {
            unit_health,
            quantity
        }
    }
}
