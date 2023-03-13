
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr};


#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, JsonSchema)]

pub struct Constants {
    pub name: String,
    pub symbol: String,
    pub max_supply: u128,
}



pub const ADMIN:  Item<Addr>  = Item::new("admin");
pub const MAXHOLDBALANCE:  Item<u128>  = Item::new("max_hold_balance");
pub const SHAREHOLDERS: Item<Vec<Addr>>=Item::new("shareholders");
pub const AUTHORISEDCOUNTRIES:Item<Vec<u128>>=Item::new("authorised_countries");
pub const FREEZEDACCOUNT :Map<&Addr,bool>=Map::new("freezed_account");