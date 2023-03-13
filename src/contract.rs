use crate::msg::{BalanceResp, QueryMsg,InstantiateMsg,FrozonBalanceResp,ExecuteMsg,ShareHoldersResp};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,Storage, to_vec, StdResult
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use crate::state::{Constants,ADMIN,SHAREHOLDERS,AUTHORISEDCOUNTRIES,FREEZEDACCOUNT};
use crate::error::ContractError;

pub const PREFIX_CONFIG: &[u8] = b"config";
pub const PREFIX_BALANCES: &[u8] = b"balances";
pub const KEY_CONSTANTS: &[u8] = b"constants";
pub const KEY_TOTAL_SUPPLY: &[u8] = b"total_supply";
pub const PREFIX_FREEZE_AMOUNT: &[u8]= b"freeze_amount";




pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response,ContractError> {
    let mut total_supply: u128 = 0;
    {
        // Initial balances
        for row in msg.initial_balances {
            let amount_raw = row.amount.u128();
            let freeze_amount_raw=row.freeze_amount.u128();
            let mut balances_store = PrefixedStorage::new(deps.storage, PREFIX_BALANCES);
            balances_store.set(row.address.as_str().as_bytes(), &amount_raw.to_be_bytes());
            let  mut freeze_balance_store=PrefixedStorage::new(deps.storage,PREFIX_FREEZE_AMOUNT);
            freeze_balance_store.set(row.address.as_str().as_bytes(), &freeze_amount_raw.to_be_bytes());
            total_supply += amount_raw;
        }
    }
    let mut config_store = PrefixedStorage::new(deps.storage, PREFIX_CONFIG);
    let constants = to_vec(&Constants {
        name: msg.name,
        symbol: msg.symbol,
        max_supply:msg.max_supply,
    })?;
    //admin info
    let admin=_info.sender;
    config_store.set(KEY_CONSTANTS, &constants);
    config_store.set(KEY_TOTAL_SUPPLY, &total_supply.to_be_bytes());
    ADMIN.save(deps.storage, &admin)?;
    //shareholder info(first shareHolder is deployer)
    let shareholders: StdResult<Vec<_>> = msg.share_holders.into_iter().map(|addr| deps.api.addr_validate(&addr)).collect();
    //authorised Countries
    let authorisedcountries:Vec<_>=msg.authorised_countries.into_iter().map(|country_code|country_code).collect();
    SHAREHOLDERS.save(deps.storage, &shareholders?)?;
    AUTHORISEDCOUNTRIES.save(deps.storage, &authorisedcountries)?;
    Ok(Response::default())
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary,ContractError> {
    use QueryMsg::*;

    match msg {
        Balance {address} => query::balanceof(deps, &address),
        FrozenBalance{address}=>query::frozenbalanceof(deps, &address),
        ShareHolders {} => query::shareholders(deps)
    }
}


pub fn execute(deps:DepsMut,env:Env,info:MessageInfo,msg:ExecuteMsg) -> Result<Response,ContractError>{
    use ExecuteMsg::*;
    match msg {
        FreezeToken { amount } =>execute::freezetoken(deps, env, info, amount),
        UnfreezeToken{amount}=>execute::unfreezetoken(deps, env, info, amount),
        Transfer{reciever,amount,countrycode}=>execute::transfer(deps, env, info, reciever,amount, countrycode),
        FreezeAccount{account}=>execute::freeze_account(deps, env, info, account),
        RemoveShareholder{account}=>execute::remove_shareholder(deps, env, info, account)
    }
}
mod query {
    use cosmwasm_std::{Addr, Uint128};

    use super::*;

    pub fn balanceof(deps:Deps,address:&String) -> Result<Binary,ContractError> {
        
        let address_key = deps.api.addr_validate(&address)?;
        let balance = read_balance(deps.storage, &address_key)?;
        let out = to_binary(&BalanceResp {
            balance: Uint128::from(balance),
        })?;
        Ok(out)
    }

    pub fn frozenbalanceof(deps:Deps,address:&String) ->Result<Binary,ContractError>{
        let address_key = deps.api.addr_validate(&address)?;
        let balance = read_frozen_balance(deps.storage, &address_key)?;
        let out = to_binary(&FrozonBalanceResp {
            frozonbalance: Uint128::from(balance),
        })?;
        Ok(out)
    }

    pub fn shareholders(deps:Deps) ->Result<Binary,ContractError>{
        let shareholderdata=SHAREHOLDERS.load(deps.storage)?;
        let shareholders=get_shareholder(&shareholderdata);
        let out=to_binary(&ShareHoldersResp{
            shareholders
        })?;
        Ok(out)

    }

    fn get_shareholder(vec: &Vec<Addr>) -> Vec<String> {
        vec.iter()
            .map(|address| address.to_string())
            .collect()
    }

    pub fn bytes_to_u128(data: &[u8]) -> Result<u128, ContractError> {
        match data[0..16].try_into() {
            Ok(bytes) => Ok(u128::from_be_bytes(bytes)),
            Err(_) => Err(ContractError::CorruptedDataFound {}),
        }
    }

    pub fn read_u128(store: &ReadonlyPrefixedStorage, key: &Addr) -> Result<u128, ContractError> {
        let result = store.get(key.as_str().as_bytes());
        match result {
            Some(data) => bytes_to_u128(&data),
            None => Ok(0u128),
        }
    }
    
    fn read_balance(store: &dyn Storage, owner: &Addr) -> Result<u128, ContractError> {
        let balance_store = ReadonlyPrefixedStorage::new(store, PREFIX_BALANCES);
        read_u128(&balance_store, owner)
    }

    fn read_frozen_balance(store: &dyn Storage, owner: &Addr) -> Result<u128, ContractError> {
        let frozen_balance_store = ReadonlyPrefixedStorage::new(store, PREFIX_FREEZE_AMOUNT);
        read_u128(&frozen_balance_store, owner)
    }
}
mod execute{
    use cosmwasm_std::{Uint128, Addr, Event};
    use super::*;

    pub fn freezetoken(deps:DepsMut,_env:Env,info:MessageInfo,amount:Uint128) ->Result<Response,ContractError>{
        let amount_raw=amount.u128();
        let mut accountbalance=read_balance(deps.storage, &info.sender)?;
        let mut freeze_balance=read_frozen_balance(deps.storage, &info.sender)?;
        if accountbalance<amount_raw{
            return Err(ContractError::InsufficientFunds { balance: accountbalance, required: amount_raw });
        }
        accountbalance-=amount_raw;
        freeze_balance+=amount_raw;
        let mut balance_store=PrefixedStorage::new(deps.storage, PREFIX_BALANCES);
        balance_store.set(info.sender.as_str().as_bytes(), &accountbalance.to_be_bytes());   
        let mut freeze_balance_store=PrefixedStorage::new(deps.storage, PREFIX_FREEZE_AMOUNT);
        freeze_balance_store.set(info.sender.as_str().as_bytes(), &freeze_balance.to_be_bytes());
        
        Ok(Response::new()
    .add_event(Event::new("freeze_amount") .add_attribute("action", "freeze_amount")))
    }

    pub fn unfreezetoken(deps:DepsMut,_env:Env,info:MessageInfo,amount:Uint128) ->Result<Response,ContractError>{
        let amount_raw=amount.u128();
        let mut accountbalance=read_balance(deps.storage, &info.sender)?;
        let mut freeze_balance=read_frozen_balance(deps.storage, &info.sender)?;
        if amount_raw>freeze_balance{
            return Err(ContractError::InsufficientFreezeFunds{ balance: freeze_balance, required: amount_raw });
        }
        accountbalance+=amount_raw;
        freeze_balance-=amount_raw;
        let mut balance_store=PrefixedStorage::new(deps.storage, PREFIX_BALANCES);
        balance_store.set(info.sender.as_str().as_bytes(), &accountbalance.to_be_bytes());   
        let mut freeze_balance_store=PrefixedStorage::new(deps.storage, PREFIX_FREEZE_AMOUNT);
        freeze_balance_store.set(info.sender.as_str().as_bytes(), &freeze_balance.to_be_bytes());
        
        Ok(Response::new()
    .add_event(Event::new("unfreeze_amount") .add_attribute("action", "unfreeze_amount")))
    }

    pub fn transfer(deps:DepsMut,_env:Env,info:MessageInfo,reciever:String,amount:Uint128,countrycode:u128) ->Result<Response,ContractError>{
        let amount_raw=amount.u128();
        let country_code=countrycode;
        let authorised_countries=AUTHORISEDCOUNTRIES.load(deps.storage)?;
        let mut shareholders=SHAREHOLDERS.load(deps.storage)?;
        let freezedaccount=FREEZEDACCOUNT.load(deps.storage, &info.sender)?;
        let reciever_addr=deps.api.addr_validate(&reciever)?;
        let mut senderbalance=read_balance(deps.storage, &info.sender)?;
        let mut recieverbalance=read_balance(deps.storage, &reciever_addr)?;
        let  freeze_balance=read_frozen_balance(deps.storage, &info.sender)?;


        if senderbalance<amount_raw{
            return Err(ContractError::InsufficientFunds { balance: senderbalance, required: amount_raw });
        }
        else if senderbalance==0 && freeze_balance>=amount_raw {
            return Err(ContractError::OnlyFeezeFundLeft { balance: freeze_balance });
        }
        else if !authorised_countries.contains(&country_code) {
            return Err(ContractError::UnauthorisedCountry { country_code: country_code });
        }
        else if freezedaccount==true {
            return Err(ContractError::FreezedAccount {});
        }
        senderbalance-=amount_raw;
        recieverbalance+=amount_raw;
        let mut balance_store=PrefixedStorage::new(deps.storage, PREFIX_BALANCES);
        balance_store.set(info.sender.as_str().as_bytes(), &senderbalance.to_be_bytes());   
        balance_store.set(reciever_addr.as_str().as_bytes(), &recieverbalance.to_be_bytes());

        shareholders.push(reciever_addr);
        
        Ok(Response::new()
        .add_event(Event::new("transfer") .add_attribute("action", "transfer")))
    }

    pub fn freeze_account(deps:DepsMut,_env:Env,_info:MessageInfo,account:String) ->Result<Response,ContractError>{
        let address=deps.api.addr_validate(&account)?;
        let  freeze_account_info=FREEZEDACCOUNT.load(deps.storage,&address)?;
        let  admin = ADMIN.load(deps.storage)?;
        if admin!=_info.sender{
            return Err(ContractError::NotAdmin {});
        }

        let value=|value:Option<bool>|->StdResult<bool>{
            match value{
                Some(_value)=>
                if _value == true{
                Ok(false)}
                else{
                    Ok(true)
                },
                None =>Ok(true),
            }
        }; 
        if freeze_account_info==true{
            return Err(ContractError::CanNotFreezeAccount {})?;
        }
        
            FREEZEDACCOUNT.update(deps.storage, &address, value)?;
            Ok(Response::new().add_event(Event::new("freeze_account")).add_attribute("action", "freeze_Account"))
        
    }

    pub fn remove_shareholder(deps:DepsMut,_env:Env,info:MessageInfo,account:String)->Result<Response,ContractError>{
        let admin=ADMIN.load(deps.storage)?;
        let address=deps.api.addr_validate(&account)?;
        let account_balance=read_balance(deps.storage, &address)?;
        if admin!=info.sender{
            return Err(ContractError::NotAdmin {  });
        }
        if account_balance>0{
            return Err(ContractError::CanNotRemove { address: address, balance: account_balance });
        }
        let mut shareholders=SHAREHOLDERS.load(deps.storage)?;
       let index=shareholders.iter().position(|x| x==&address);
       match index{
        Some(i) => Some(shareholders.remove(i)),
        None=>None,
       };
        

        Ok(Response::new().add_event(Event::new("remove_shareholder")).add_attribute("action", "remove_shareholder"))
    }

    pub fn bytes_to_u128(data: &[u8]) -> Result<u128, ContractError> {
        match data[0..16].try_into() {
            Ok(bytes) => Ok(u128::from_be_bytes(bytes)),
            Err(_) => Err(ContractError::CorruptedDataFound {}),
        }
    }

    pub fn read_u128(store: &ReadonlyPrefixedStorage, key: &Addr) -> Result<u128, ContractError> {
        let result = store.get(key.as_str().as_bytes());
        match result {
            Some(data) => bytes_to_u128(&data),
            None => Ok(0u128),
        }
    }
    
    fn read_balance(store: &dyn Storage, owner: &Addr) -> Result<u128, ContractError> {
        let balance_store = ReadonlyPrefixedStorage::new(store, PREFIX_BALANCES);
        read_u128(&balance_store, owner)
    }

    fn read_frozen_balance(store: &dyn Storage, owner: &Addr) -> Result<u128, ContractError> {
        let frozen_balance_store = ReadonlyPrefixedStorage::new(store, PREFIX_FREEZE_AMOUNT);
        read_u128(&frozen_balance_store, owner)
    }

    

    
}