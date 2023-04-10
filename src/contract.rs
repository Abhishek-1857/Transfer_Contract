use crate::msg::{BalanceResp, QueryMsg,InstantiateMsg,FrozonBalanceResp,ExecuteMsg,ShareHoldersResp,MigrateMsg};
use cosmwasm_std::{
    to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response,Storage, to_vec, StdResult,entry_point, Empty
};
use cosmwasm_storage::{PrefixedStorage, ReadonlyPrefixedStorage};
use crate::state::{Constants,ADMIN,SHAREHOLDERS,AUTHORISEDCOUNTRIES,FREEZEDACCOUNT,MAXHOLDBALANCE};
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
        // Initial balances,and freezeBalance
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
    //MaxHoldBalance beside admin
    let maxholdbal=msg.max_hold_balance;
    MAXHOLDBALANCE.save(deps.storage, &maxholdbal)?;

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

    //To check balance of provided address 
    pub fn balanceof(deps:Deps,address:&String) -> Result<Binary,ContractError> {
        
        let address_key = deps.api.addr_validate(&address)?;
        let balance = read_balance(deps.storage, &address_key)?;
        let out = to_binary(&BalanceResp {
            balance: Uint128::from(balance),
        })?;
        Ok(out)
    }

    //To check frozen token balance of address
    pub fn frozenbalanceof(deps:Deps,address:&String) ->Result<Binary,ContractError>{
        let address_key = deps.api.addr_validate(&address)?;
        let balance = read_frozen_balance(deps.storage, &address_key)?;
        let out = to_binary(&FrozonBalanceResp {
            frozonbalance: Uint128::from(balance),
        })?;
        Ok(out)
    }

    //To get shareholders list
    pub fn shareholders(deps:Deps) ->Result<Binary,ContractError>{
        let shareholderdata=SHAREHOLDERS.load(deps.storage)?;
        let shareholders=get_shareholder(&shareholderdata);
        let out=to_binary(&ShareHoldersResp{
            shareholders
        })?;
        Ok(out)

    }

    pub fn get_shareholder(vec: &Vec<Addr>) -> Vec<String> {
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

    //To freeze some amount of tokens
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


    //To unfreeze tokens
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
  
    //Transfer token and doing neccessary checks  
    pub fn transfer(deps:DepsMut,_env:Env,info:MessageInfo,reciever:String,amount:Uint128,countrycode:u128) ->Result<Response,ContractError>{
        let amount_raw=amount.u128();
        let country_code=countrycode;
        let authorised_countries=AUTHORISEDCOUNTRIES.load(deps.storage)?;
        //Loading shareholders data
        let mut shareholders=SHAREHOLDERS.load(deps.storage)?;
        //Getting if the account is able to transfer or freezed by admin
        let freezedaccount=FREEZEDACCOUNT.load(deps.storage, &info.sender).unwrap_or(false);
        //Getting maximum holder balance
        let maxholdbalance=MAXHOLDBALANCE.load(deps.storage)?;
        let reciever_addr=deps.api.addr_validate(&reciever)?;
        //Getting sender and reciever balances and freeze balance of sender
        let mut senderbalance=read_balance(deps.storage, &info.sender)?;
        let mut recieverbalance=read_balance(deps.storage, &reciever_addr)?;
        let  freeze_balance=read_frozen_balance(deps.storage, &info.sender)?;

        //CHECKS
        if senderbalance<amount_raw{
            return Err(ContractError::InsufficientFunds { balance: senderbalance, required: amount_raw });
        }
        //Checking if he have zero token left and only has freeze funds left(CAN'T TRANSFER)
        else if senderbalance==0 && freeze_balance>=amount_raw {
            return Err(ContractError::OnlyFeezeFundLeft { balance: freeze_balance });
        }
        //Checking if the coutry code is authorised or not
        else if !authorised_countries.contains(&country_code) {
            return Err(ContractError::UnauthorisedCountry { country_code });
        }
        //Checking if the account is frozen or not
        else if freezedaccount==true {
            return Err(ContractError::FreezedAccount {});
        }
        senderbalance-=amount_raw;
        recieverbalance+=amount_raw;

        //Checking if the reciever balance is greater than the maximum holder balance after transfering if so then revert
        if recieverbalance>maxholdbalance{
            recieverbalance-=amount_raw;
            let mut balance_store=PrefixedStorage::new(deps.storage, PREFIX_BALANCES);
            balance_store.set(reciever_addr.as_str().as_bytes(), &recieverbalance.to_be_bytes());
            return Err(ContractError::MaxHolderBalance { address: reciever_addr})?;
        }
        //Updating Balances of sender and reciever
        let mut balance_store=PrefixedStorage::new(deps.storage, PREFIX_BALANCES);
        balance_store.set(info.sender.as_str().as_bytes(), &senderbalance.to_be_bytes());   
        balance_store.set(reciever_addr.as_str().as_bytes(), &recieverbalance.to_be_bytes());
 
         //Pushing the sender into shareholders list
         shareholders.push(reciever_addr);
         SHAREHOLDERS.save(deps.storage, &shareholders)?;
        
        Ok(Response::new()
        .add_event(Event::new("transfer") .add_attribute("action", "transfer")))
    }


     // To freeze account(Only be done by the admin)
    pub fn freeze_account(deps:DepsMut,_env:Env,_info:MessageInfo,account:String) ->Result<Response,ContractError>{
        let address=deps.api.addr_validate(&account)?;
        let  freeze_account_info=FREEZEDACCOUNT.load(deps.storage,&address).unwrap_or(false);
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
        // freeze_account_info=true;
            FREEZEDACCOUNT.update(deps.storage, &address, value)?;
            Ok(Response::new().add_event(Event::new("freeze_account")).add_attribute("action", "freeze_Account"))
        
    }
    
    //Removing Shareholder(Only be done by Admin)
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
       SHAREHOLDERS.save(deps.storage, &shareholders)?;
        

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


    pub fn get_shareholder(vec: &Vec<Addr>) -> Vec<String> {
        vec.iter()
            .map(|address| address.to_string())
            .collect()
    }    

    
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    // perform state update or anything neccessary for the migration
    Ok(Response::default())
}

// #[cfg_attr(not(feature = "library"), entry_point)]
// pub fn get_updated_balance(_deps: Deps, _env: Env, _msg: Empty) -> Result<Response,ContractError> {
//   query::balanceof(_deps, &"tp17kjvwvfjdf3j9knr72rewx94scqjgjf3gmz7tx".to_string())?;
//     Ok(Response::default())
// }

#[cfg(test)]
mod tests{
    use super::*;
    use crate::msg::InitialBalance;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_slice, Addr, Env, MessageInfo, Storage, Timestamp, Uint128};
    use cosmwasm_storage::ReadonlyPrefixedStorage;

    fn mock_env_height(signer: &str, height: u64, time: u64) -> (Env, MessageInfo) {
        let mut env = mock_env();
        let info = mock_info(signer, &[]);
        env.block.height = height;
        env.block.time = Timestamp::from_seconds(time);
        (env, info)
    }

    fn get_constants(storage: &dyn Storage) -> Constants {
        let config_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_CONFIG);
        let data = config_storage
            .get(KEY_CONSTANTS)
            .expect("no config data stored");
        from_slice(&data).expect("invalid data")
    }

    fn get_total_supply(storage: &dyn Storage) -> u128 {
        let config_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_CONFIG);
        let data = config_storage
            .get(KEY_TOTAL_SUPPLY)
            .expect("no decimals data stored");
        return execute::bytes_to_u128(&data).unwrap();
    }

    fn get_balance(storage: &dyn Storage, address: &Addr) -> u128 {
        let balances_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_BALANCES);
        return execute::read_u128(&balances_storage, address).unwrap();
    }

    fn get_frozen_balance(storage: &dyn Storage, address: &Addr) -> u128 {
        let frozen_balances_storage = ReadonlyPrefixedStorage::new(storage, PREFIX_FREEZE_AMOUNT);
        return execute::read_u128(&frozen_balances_storage, address).unwrap();
    }

    fn get_shareholder(storage: &dyn Storage) -> Result<Vec<String>,ContractError> {
        let shareholder =   SHAREHOLDERS.load(storage)?;
        return Ok(execute::get_shareholder(&shareholder));
    }


    mod instantiate {
        use super::*;
        #[test]
        fn works() {
            let mut deps = mock_dependencies();
            let instantiate_msg = InstantiateMsg {
                name: "Provenance Token".to_string(),
                symbol: "PRV".to_string(),
                max_supply:1000000,
                initial_balances: [InitialBalance {
                    address: "creator".to_string(),
                    amount: Uint128::from(11223344u128),
                    freeze_amount:Uint128::from(100u128)
                }]
                .to_vec(),
                share_holders: ["creator".to_string()].to_vec(),
                authorised_countries:[91].to_vec(),
                max_hold_balance: 10000,
            };
            let (env, info) = mock_env_height("creator", 450, 550);
            let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
            assert_eq!(0, res.messages.len());
            assert_eq!(
                get_constants(&deps.storage),
                Constants {
                    name: "Provenance Token".to_string(),
                    symbol: "PRV".to_string(),
                    max_supply: 1000000,
                }
            );
            assert_eq!(
                get_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
                11223344
            );
            assert_eq!(get_total_supply(&deps.storage), 11223344);
            assert_eq!(get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),100);
            assert_eq!(get_shareholder(&deps.storage),Ok(["creator".to_string()].to_vec()));
        }

}

mod transfer {
    use super::*;
    use cosmwasm_std::Event;

    fn make_instantiate_msg() -> InstantiateMsg {
        InstantiateMsg {
            name: "Provenance Token".to_string(),
            symbol: "PRV".to_string(),
            max_supply:1000000,
            initial_balances: [InitialBalance {
                address: "creator".to_string(),
                amount: Uint128::from(11223344u128),
                freeze_amount:Uint128::from(100u128)
            }]
            .to_vec(),
            share_holders: ["creator".to_string(),"creator2".to_string()].to_vec(),
            authorised_countries:[91].to_vec(),
            max_hold_balance: 10000,
        }
    }

    #[test]
    fn transfer_to_address() {
        let mut deps = mock_dependencies();
        let instantiate_msg = make_instantiate_msg();
        let (env, info) = mock_env_height("creator", 450, 550);
        let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        // Initial state
        assert_eq!(
            get_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
            11223344
        );
        assert_eq!(
            get_balance(&deps.storage, &Addr::unchecked("addr1111".to_string())),
            0
        );
      
        assert_eq!(get_total_supply(&deps.storage), 11223344);
        // Transfer
        let transfer_msg = ExecuteMsg::Transfer {
            reciever: "addr1111".to_string(),
            amount:Uint128::from(1u128),
            countrycode: 91,
        };
        let (env, info) = mock_env_height("creator", 450, 550);
        let transfer_result = execute(deps.as_mut(), env, info, transfer_msg).unwrap();
        assert_eq!(transfer_result.messages.len(), 0);
        let expected_event = Event::new("transfer")
        .add_attribute("action","transfer");
        

    // Verify the response
    assert_eq!(
        transfer_result,
        Response::new()
            .add_event(expected_event.clone())
    );

    // Verify the emitted event
    let events = transfer_result.events.clone();
    assert_eq!(1, events.len()); // Ensure there is only one event emitted
    assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
        // New state
        assert_eq!(
            get_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
            11223343
        ); // -1
        assert_eq!(
            get_balance(&deps.storage, &Addr::unchecked("addr1111".to_string())),
            1
        ); // +1
        
        assert_eq!(get_total_supply(&deps.storage), 11223344);
    }



    #[test] 
    fn freeze_token() {
        let mut deps = mock_dependencies();
        let instantiate_msg = make_instantiate_msg();
        let (env, info) = mock_env_height("creator", 450, 550);
        let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        // Initial state
        assert_eq!(
            get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
            100
        );
        
        assert_eq!(get_total_supply(&deps.storage), 11223344);
        // Transfer
        let freeze_token_message = ExecuteMsg::FreezeToken {
            amount:Uint128::from(1u128),
        };
        let (env, info) = mock_env_height("creator", 450, 550);
        let freeze_token_result = execute(deps.as_mut(), env, info, freeze_token_message).unwrap();
        assert_eq!(freeze_token_result.messages.len(), 0);
        let expected_event = Event::new("freeze_amount")
        .add_attribute("action","freeze_amount");
        

    // Verify the response
    assert_eq!(
        freeze_token_result,
        Response::new()
            .add_event(expected_event.clone())
    );

    // Verify the emitted event
    let events = freeze_token_result.events.clone();
    assert_eq!(1, events.len()); // Ensure there is only one event emitted
    assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
        // New state
        assert_eq!(
            get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
            101
        ); // +1
       
        
        assert_eq!(get_total_supply(&deps.storage), 11223344);
    }

    #[test] 
    fn un_freeze_token() {
        let mut deps = mock_dependencies();
        let instantiate_msg = make_instantiate_msg();
        let (env, info) = mock_env_height("creator", 450, 550);
        let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        // Initial state
        assert_eq!(
            get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
            100
        );
        
        assert_eq!(get_total_supply(&deps.storage), 11223344);
        // Transfer
        let unfreeze_token_message = ExecuteMsg::UnfreezeToken{
            amount:Uint128::from(1u128),
        };
        let (env, info) = mock_env_height("creator", 450, 550);
        let unfreeze_token_result = execute(deps.as_mut(), env, info, unfreeze_token_message).unwrap();
        assert_eq!(unfreeze_token_result.messages.len(), 0);
        let expected_event = Event::new("unfreeze_amount")
        .add_attribute("action","unfreeze_amount");
        

    // Verify the response
    assert_eq!(
        unfreeze_token_result,
        Response::new()
            .add_event(expected_event.clone())
    );

    // Verify the emitted event
    let events = unfreeze_token_result.events.clone();
    assert_eq!(1, events.len()); // Ensure there is only one event emitted
    assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
        // New state
        assert_eq!(
            get_frozen_balance(&deps.storage, &Addr::unchecked("creator".to_string())),
            99
        ); // -1
       
        
        assert_eq!(get_total_supply(&deps.storage), 11223344);
    }

    #[test] 
    fn remove_shareholder() {
        let mut deps = mock_dependencies();
        let instantiate_msg = make_instantiate_msg();
        let (env, info) = mock_env_height("creator", 450, 550);
        let res = instantiate(deps.as_mut(), env, info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        // Initial state
        assert_eq!(get_shareholder(&deps.storage),Ok(["creator".to_string(),"creator2".to_string()].to_vec()));
        
        // Transfer
        let remove_shareholder_message = ExecuteMsg::RemoveShareholder{
           account:"creator2".to_string(),
        };
        let (env, info) = mock_env_height("creator", 450, 550);
        let remove_shareholder_result = execute(deps.as_mut(), env, info, remove_shareholder_message).unwrap();
        assert_eq!(remove_shareholder_result.messages.len(), 0);
    //     let expected_event = Event::new("remove_shareholder")
    //     .add_attribute("action","remove_shareholder");
    // // Verify the response
    // assert_eq!(
    //     remove_shareholder_result,
    //     Response::new()
    //         .add_event(expected_event.clone())
    // );
    // // Verify the emitted event
    // let events = remove_shareholder_result.events.clone();
    // assert_eq!(1, events.len()); // Ensure there is only one event emitted
    // assert_eq!(expected_event, events[0]); // Ensure the emitted event matches the expected event
        // New state
        assert_eq!(get_shareholder(&deps.storage),Ok(["creator".to_string()].to_vec()));
    }  
}
mod query {
    use super::*;
    use cosmwasm_std::{attr, Addr};


    fn make_instantiate_msg() -> InstantiateMsg {
        InstantiateMsg {
            name: "Provenance Token".to_string(),
            symbol: "PRV".to_string(),
            max_supply:1000000,
            initial_balances: [InitialBalance {
                address: "creator".to_string(),
                amount: Uint128::from(11223344u128),
                freeze_amount:Uint128::from(100u128)
            }]
            .to_vec(),
            share_holders: ["creator".to_string(),"creator2".to_string()].to_vec(),
            authorised_countries:[91].to_vec(),
            max_hold_balance: 10000,
        }
    }

    #[test]
    fn can_query_balance_of_existing_address() {
        let mut deps = mock_dependencies();
        let instantiate_msg = make_instantiate_msg();
        let (env, info) = mock_env_height("creator", 450, 550);
        let res = instantiate(deps.as_mut(), env.clone(), info, instantiate_msg).unwrap();
        assert_eq!(0, res.messages.len());
        let query_msg = QueryMsg::Balance {
            address: "creator".to_string(),
        };
        let query_result = query(deps.as_ref(), env, query_msg).unwrap();
        assert_eq!(query_result.as_slice(), b"{\"balance\":\"11223344\"}");
    }

}
}