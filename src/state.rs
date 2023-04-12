
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};
use cosmwasm_std::{Addr};


#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, JsonSchema)]

pub struct Constants {
    pub name: String,
}
#[derive(Serialize, Debug, Deserialize, Clone, PartialEq, JsonSchema)]

pub struct MintRequestInfo{
    pub proposal_id:u128,
    pub owner : Addr,
    pub approvals:u128,
    pub completed : bool
}

pub const SIGNERS:Item<Vec<Addr>>=Item::new("signers");
pub const ADMIN:  Item<Addr>  = Item::new("admin");
pub const MINT_REQUEST_MAPPING: Map<u128,MintRequestInfo>=Map::new("mint_request_info");
pub const PROPOSAL_ID:Item<u128>=Item::new("proposal_id");
pub const HAS_APPROVED:Map<(Addr,u128),bool>=Map::new("has_voted");
// pub const MAXHOLDBALANCE:  Item<u128>  = Item::new("max_hold_balance");
// pub const SHAREHOLDERS: Item<Vec<Addr>>=Item::new("shareholders");
// pub const AUTHORISEDCOUNTRIES:Item<Vec<u128>>=Item::new("authorised_countries");
// pub const FREEZEDACCOUNT :Map<&Addr,bool>=Map::new("freezed_account");