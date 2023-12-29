use crate::unit_health::UnitHealth;

pub struct InactiveUnitStack {
    pub unit_health: UnitHealth,
    pub quantity: u8,
    pub stack_after_hit: Option<Box<InactiveUnitStack>>,  // Use Box<InactiveUnitStack> here
}

impl InactiveUnitStack {
    pub fn new(unit_health: UnitHealth, quantity: u8, stack_after_hit: Option<Box<InactiveUnitStack>>) -> Self {
        Self {
            unit_health,
            quantity,
            stack_after_hit,
        }
    }    
}