use serde::{Deserialize, Serialize,};
use cosmwasm_std::Uint128;
use schemars::JsonSchema;


#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct InitialBalance {
    pub address: String,
    pub amount: Uint128,
    pub freeze_amount:Uint128
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]

pub struct InstantiateMsg{
    pub name : String,
    pub symbol:String,
    pub max_supply :u128,
    pub initial_balances: Vec<InitialBalance>,
    pub share_holders:Vec<String>,
    pub authorised_countries:Vec<u128>   
    
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum ExecuteMsg{
    FreezeToken {amount :Uint128},
    UnfreezeToken {amount :Uint128},
    Transfer {reciever:String,amount:Uint128,countrycode:u128},
    FreezeAccount {account :String},
    RemoveShareholder {account :String}
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub enum QueryMsg {
   Balance {address :String},
   FrozenBalance {address : String},
   ShareHolders {}
   
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct BalanceResp {
    pub balance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct FrozonBalanceResp {
    pub frozonbalance: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
pub struct ShareHoldersResp {
    pub shareholders: Vec<String>,
}




