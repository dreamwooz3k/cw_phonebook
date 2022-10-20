use cosmwasm_std::Addr;
use cw_storage_plus::Map;

pub const PHONEBOOK: Map<Addr, String> = Map::new("Phone Book");
