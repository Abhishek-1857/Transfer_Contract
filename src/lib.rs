use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response,
};
use msg::ExecuteMsg;

pub mod contract;
pub mod msg;
pub mod state;
pub mod error;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(deps: DepsMut, env: Env, info: MessageInfo, msg: msg::InstantiateMsg)
  -> Result<Response,error::ContractError>
{
    contract::instantiate(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: msg::QueryMsg)
  -> Result<Binary,error::ContractError>
{
   contract::query(deps, env, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> Result<Response,error::ContractError>{
    contract::execute(deps, env, info, msg)
}