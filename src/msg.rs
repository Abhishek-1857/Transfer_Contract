use cosmwasm_std::Uint128;
use  cosmwasm_schema::{cw_serde,QueryResponses};

#[cw_serde]
pub struct InitialBalance {
    pub address: String,
    pub amount: Uint128,
    pub freeze_amount:Uint128
}
#[cw_serde]

pub struct InstantiateMsg{
    pub name : String,
    pub symbol:String,
    pub max_supply :u128,
    pub initial_balances: Vec<InitialBalance>,
    pub share_holders:Vec<String>,
    pub authorised_countries:Vec<u128>,
    pub max_hold_balance :u128  
    
}

#[cw_serde]
pub enum ExecuteMsg{
    FreezeToken {amount :Uint128},
    UnfreezeToken {amount :Uint128},
    Transfer {reciever:String,amount:Uint128,countrycode:u128},
    FreezeAccount {account :String},
    RemoveShareholder {account :String}
}

#[cw_serde]
#[derive(QueryResponses)]

pub enum QueryMsg {
    #[returns(BalanceResp)]
   Balance {address :String},
   #[returns(FrozonBalanceResp)]
   FrozenBalance {address : String},
   #[returns(ShareHoldersResp)]
   ShareHolders {}
   
}

#[cw_serde]
pub struct BalanceResp {
    pub balance: Uint128,
}

#[cw_serde]
pub struct FrozonBalanceResp {
    pub frozonbalance: Uint128,
}

#[cw_serde]
pub struct ShareHoldersResp {
    pub shareholders: Vec<String>,
}




