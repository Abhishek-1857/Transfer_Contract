

use cosmwasm_std::{Addr,Uint128};
use  cosmwasm_schema::{cw_serde,QueryResponses};

#[cw_serde]

pub struct InstantiateMsg{
    pub name : String,
    pub signers:Vec<Addr>
    
}

#[cw_serde]
pub enum ExecuteMsg{
   CreateMintRequest {},
   ApproveMintRequest{proposal_id : u128},
   Mint{proposal_id:u128}
}

#[cw_serde]
#[derive(QueryResponses)]

pub enum QueryMsg {
//     #[returns(BalanceResp)]
//    Balance {address :String},
//    #[returns(FrozonBalanceResp)]
//    FrozenBalance {address : String},
//    #[returns(ShareHoldersResp)]
//    ShareHolders {}
   
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



