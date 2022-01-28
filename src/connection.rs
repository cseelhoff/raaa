use crate::territory::Territory;

#[derive(Debug)]
pub struct Connection {
    pub source_index: u8,
    pub destination_index: u8,
    pub required_territories: Vec<u8>
}

pub(crate) fn create_connections(territories: &mut Vec<Territory>) {
    
    let con_russia_germany: Connection = Connection{
        source_index: 0,
        destination_index: 1,
        required_territories: Vec::new()
    };

    let con_germany_russia: Connection = Connection{
        source_index: 1,
        destination_index: 0,
        required_territories: Vec::new()
    };

    territories[0].connections.push(con_russia_germany);
    territories[1].connections.push(con_germany_russia);
}