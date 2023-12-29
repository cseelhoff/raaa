use crate::territory::Territory;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Connection {
    pub source_name: &'static str,
    pub destination_name: &'static str,
    pub required_names: Vec<&'static str>,
    pub source_territory: Territory,
    pub destination_territory: Territory,
    pub required_territories: Vec<Territory>
}
