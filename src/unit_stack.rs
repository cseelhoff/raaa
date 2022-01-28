use super::unit::Unit;

pub struct UnitStack {
    unit: Unit,
    //quantity: u8,
    data: Data,
}

#[derive(Default, Debug, PartialEq)]
pub struct Data {
    quantity: u8
}

#[derive(Default, Debug, PartialEq)]
pub struct UnitStacks {
    //infantry: crate::unit_stack::Data,
    artillery: crate::unit_stack::Data,
    /*
    armour: crate::unit_stack::Data,
    fighter: crate::unit_stack::Data,
    bomber: crate::unit_stack::Data,
    aa_gun: crate::unit_stack::Data,
    transport: crate::unit_stack::Data,
    submarine: crate::unit_stack::Data,
    destroyer: crate::unit_stack::Data,
    crusier: crate::unit_stack::Data,
    carrier: crate::unit_stack::Data,
    battleship: crate::unit_stack::Data,
    */
}
